import Config

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :webrtclixir, WebrtclixirWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base: "Cb8bpze5thhoW2HMgvJjYFSXI00vwDdrturAcJppKe2mg/usLcLz9/Bk5dI/+n0c",
  server: false

# In test we don't send emails
config :webrtclixir, Webrtclixir.Mailer, adapter: Swoosh.Adapters.Test

# Disable swoosh api client as it is only required for production adapters
config :swoosh, :api_client, false

# Print only warnings and errors during test
config :logger, level: :warning

# Initialize plugs at runtime for faster test compilation
config :phoenix, :plug_init_mode, :runtime
