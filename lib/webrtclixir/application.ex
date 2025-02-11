defmodule Webrtclixir.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      WebrtclixirWeb.Telemetry,
      {DNSCluster, query: Application.get_env(:webrtclixir, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Webrtclixir.PubSub},
      # Start a worker by calling: Webrtclixir.Worker.start_link(arg)
      # {Webrtclixir.Worker, arg},
      # Start to serve requests, typically the last entry
      WebrtclixirWeb.Endpoint,
      WebrtclixirWeb.Presence,
      Webrtclixir.PeerSupervisor,
      Webrtclixir.Room,
      {Registry, name: Webrtclixir.PeerRegistry, keys: :unique},
      {Finch, name: WebrtclixirFinch}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Webrtclixir.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    WebrtclixirWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
