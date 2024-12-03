defmodule Webrtclixir.HealthPlug do
  import Plug.Conn

  @behaviour Plug

  @impl true
  def init(opts), do: opts

  @impl true
  def call(%Plug.Conn{request_path: "/health"} = conn, _opts) do
    conn
    |> send_resp(200, "OK")
    |> halt()
  end

  @impl true
  def call(conn, _opts), do: conn
end