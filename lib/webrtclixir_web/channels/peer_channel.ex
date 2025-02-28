defmodule WebrtclixirWeb.PeerChannel do
  @moduledoc false

  use WebrtclixirWeb, :channel

  require Logger

  alias ExWebRTC.PeerConnection
  alias Webrtclixir.{Peer, Room}
  alias WebrtclixirWeb.Presence

  intercept ["device_event"]

  @spec send_offer(GenServer.server(), String.t()) :: :ok
  def send_offer(channel, offer) do
    Logger.info("Sending offer to peer")
    GenServer.cast(channel, {:offer, offer})
  end

  @spec send_candidate(GenServer.server(), String.t()) :: :ok
  def send_candidate(channel, candidate) do
    GenServer.cast(channel, {:candidate, candidate})
  end

  @spec close(GenServer.server()) :: :ok
  def close(channel) do
    try do
      GenServer.stop(channel, :shutdown)
    catch
      _exit_or_error, _e -> :ok
    end

    :ok
  end

  @impl true
  def join("peer:signalling-" <> channel_id, %{"in" => on, "id" => id}, socket) when on == :false do
    Logger.error("new watcher #{id}}")
    pid = self()
    send(pid, :after_join_watcher)
    {:ok, assign(socket, peer: id, channel: channel_id, in: on)}
  end

  @impl true
  def join("peer:signalling-" <> channel_id, payload, socket) do
    Logger.error("Joining #{inspect(payload)}")
    pid = self()
    send(pid, :after_join)
    case Room.add_peer(pid, payload["id"]) do
      {:ok, id} -> {:ok, assign(socket, peer: id, channel: channel_id, username: payload["username"], in: payload["in"])}
      {:error, _reason} = error -> error
    end
  end

  @impl true
  def terminate(reason, arg1) do
    Logger.error("Terminating peer channel: #{inspect(reason)}")
    :ok
  end

  @impl true
  def handle_in("sdp_answer", %{"body" => body}, socket) do
    :ok = Peer.apply_sdp_answer(socket.assigns.peer, body)
    {:noreply, socket}
  end

  @impl true
  def handle_in("sdp_offer", %{"body" => _body}, socket) do
    # TODO: renegotiate
    :ok = Peer.offer(socket.assigns.peer)
    Logger.warning("Ignoring SDP offer sent by peer #{socket.assigns.peer}")
    {:noreply, socket}
  end

  @impl true
  def handle_in("ice_candidate", %{"body" => body}, socket) do
    Peer.add_ice_candidate(socket.assigns.peer, body)
    {:noreply, socket}
  end

  @impl true
  def handle_in("device_event", body, socket) do
    Peer.device_event(socket.assigns.peer, body)
    broadcast!(socket, "device_event", body)
    {:noreply, socket}
  end

  @impl true
  def handle_out("device_event",%{"device" => _device, "event" => _event, "user_id" => user_id} = payload, socket) when user_id != socket.assigns.peer and socket.assigns.in  do
    pid = self()
    Logger.info("Receving device_event from #{inspect(pid)} with payload #{inspect(payload)}")
    Peer.set_outbound_tracks(socket.assigns.peer, payload)
    {:noreply, socket}
  end

  @impl true
  def handle_out(event,payload, socket)  do
    Logger.info("Catching #{inspect(event)} with payload #{inspect(payload)} doing nothing...")

    {:noreply, socket}
  end
  @impl true
  def handle_cast({:offer, sdp_offer}, socket) do
    Logger.info("Sending last offer to peer")
    push(socket, "sdp_offer", %{"body" => sdp_offer})
    {:noreply, socket}
  end

  @impl true
  def handle_cast({:candidate, candidate}, socket) do
    push(socket, "ice_candidate", %{"body" => candidate})
    {:noreply, socket}
  end

  @impl true
  def handle_info(:after_join, socket) do
    Logger.info("adding to streamers #{socket.assigns.peer}")
    {:ok, _ref} = Presence.track(socket, socket.assigns.peer, %{user: %{id: socket.assigns.peer, audio: nil, video: nil, channel: socket.assigns.channel, username: socket.assigns.username}})
    push(socket, "presence_state", Presence.list(socket))
    {:noreply, socket}
  end

  @impl true
  def handle_info(:after_join_watcher, socket) do
    Logger.info("adding to watcher #{socket.assigns.peer}")
    {:ok, _ref} = Presence.track(socket, socket.assigns.peer, %{user: %{watcher: true}})
    push(socket, "presence_state", Presence.list(socket))
    {:noreply, socket}
  end

  @impl true
  def handle_info(track, socket) do
    Logger.warning("adding track to #{socket.assigns.peer}")
    {:ok, _ref} = Presence.update(socket, socket.assigns.peer, %{user: %{channel: socket.assigns.channel, username: socket.assigns.username, id: socket.assigns.peer, outbounds: track.outbound, video: track.video, audio: track.audio}})
    push(socket, "presence_state", Presence.list(socket))
    {:noreply, socket}
  end
end
