use anyhow::{Context, Result};
use async_std::net::TcpStream;
use tiberius::{AuthMethod, Client, Config};
use tide::log;

#[derive(Debug, Clone)]
pub struct MSSQL;

impl MSSQL {
    /// Connects to a MSSQL database server (configured via environment variables)
    /// and returns a `tiberius::Client` for that server.
    pub async fn connect(&self) -> Result<Client<TcpStream>> {
        let mssql_host = std::env::var("MSSQL_HOST").context("Failed to configure MSSQL_HOST")?;
        let mssql_port = std::env::var("MSSQL_PORT").context("Failed to configure MSSQL_PORT")?;
        let mssql_user = std::env::var("MSSQL_USER").context("Failed to configure MSSQL_USER")?;
        let mssql_password =
            std::env::var("MSSQL_PASSWORD").context("Failed to configure MSSQL_PASSWORD")?;

        log::info!("Connecting to MSSQL Server: {}:{}.", mssql_host, mssql_port);

        let mut config = Config::new();
        config.host(mssql_host.clone());
        config.port(
            mssql_port
                .parse::<u16>()
                .context("Failed to configure MSSQL_PORT")?,
        );
        config.authentication(AuthMethod::sql_server(mssql_user, mssql_password));
        let tcp_stream = TcpStream::connect(config.get_addr()).await?;
        // We'll disable the Nagle algorithm. Buffering is handled internally with a `Sink`.
        tcp_stream.set_nodelay(true)?;
        let client = Client::connect(config, tcp_stream).await?;

        log::info!(
            "Successfully connected to MSSQL Server: {}:{}",
            mssql_host,
            mssql_port
        );

        Ok(client)
    }
}
