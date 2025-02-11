defmodule WebrtclixirWeb.UserSocket do
  use Phoenix.Socket

  channel "stream:*", WebrtclixirWeb.StreamChannel
  channel "peer:*", WebrtclixirWeb.PeerChannel

  @impl true
  def connect(%{"token" => token, "user" => user, "server" => server } = payload, socket, _connect_info) do
    case Webrtclixir.Auth.verify_token(%{user: user, server: server}, token) do
      {:ok} ->
        {:ok, assign(socket, :user_id, generate_id())}
      {:error, _reason} ->
        :error
    end
  end

  @impl true
  def id(socket), do: "user_socket:#{socket.assigns.user_id}"

  defp generate_id do
    10
    |> :crypto.strong_rand_bytes()
    |> Base.url_encode64()
  end

  # Example token verification function
  defp verify_token(token) do
    # Replace this with your actual token verification logic
    # For example, using Guardian or a similar library
    case WebrtclixirWeb.Guardian.decode_and_verify(token) do
      {:ok, claims} -> {:ok, claims["sub"]}
      {:error, _reason} = error -> error
    end
  end
end