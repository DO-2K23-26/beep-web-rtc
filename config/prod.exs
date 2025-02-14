import Config

config :webrtclixir, WebrtclixirWeb.Endpoint,
       # Binding to loopback ipv4 address prevents access from other machines.
       # Change to `ip: {0, 0, 0, 0}` to allow access from other machines.
       http: [ip: {0, 0, 0, 0}, port: 4000],
       check_origin: false,
       code_reloader: true,
       debug_errors: true,
       secret_key_base: "+2JV/8LUQgSe1Je0tHvL4Iz2reT54I3oCSTg/VxoauvXBUyBUqUAK3QS5nhluMlU",
       watchers: []

# Configures Swoosh API Client
config :swoosh, api_client: Swoosh.ApiClient.Finch, finch_name: Webrtclixir.Finch

# Disable Swoosh Local Memory Storage
config :swoosh, local: false

# Do not print debug messages in production
config :logger, level: :info

# Runtime production configuration, including reading
# of environment variables, is done on config/runtime.exs.
config :webrtclixir, :auth,
       url: "http://localhost:3333"