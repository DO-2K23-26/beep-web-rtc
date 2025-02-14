defmodule Webrtclixir.Auth do
  require Logger

  def verify_token(params, token) do
    headers = [
      {"Authorization", "Bearer #{token}"},
      {"Content-Type", "application/json"}
    ]
    url = get_auth_url() <> "/servers/" <> params[:server]
    Logger.info("trying #{inspect(url)}")

    Finch.build(:get, url, headers)
    |> Finch.request(WebrtclixirFinch)
    |> handle_response()
  end

  defp handle_response({:ok, %Finch.Response{status: 200, body: body}}) do
      {:ok}
  end

  defp handle_response({:ok, %Finch.Response{status: 401}}) do
    {:error, :unauthorized}
  end

  defp handle_response(_) do
    {:error, :server_error}
  end

  defp get_auth_url() do
    # Access the configuration for Webrtclixir.Auth
    config = Application.get_env(:webrtclixir, Webrtclixir.Auth)

    # Fetch the `url` key from the configuration
    Keyword.get(config, :url)
  end
end