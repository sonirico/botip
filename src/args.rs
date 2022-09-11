#[derive(Debug, Clone)]
pub enum Env {
    Local,
    Prod,
}

impl Env {
    pub fn parse() -> Env {
        if std::env::var("BOTIP_ENV")
            .unwrap_or("prod".to_string())
            .eq("prod")
        {
            Env::Prod
        } else {
            Env::Local
        }
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub env: Env,
    pub bind_addr: String,
    pub bind_port: u16,
    pub lookup_headers: Vec<String>,
}

impl Args {
    pub fn inspect(&self) {
        dbg!(self);
    }
    pub fn parse() -> Self {
        let mut lookup_headers: Vec<String> = std::env::var("BOTIP_LOOKUP_HEADERS")
            .unwrap_or_default()
            .split(",")
            .filter_map(|item| {
                if item.is_empty() {
                    None
                } else {
                    Some(item.to_string())
                }
            })
            .collect();

        if lookup_headers.is_empty() {
            lookup_headers = vec!["x-forwarded-for".to_string(), "x-real-ip".to_string()];
        }

        Self {
            env: Env::parse(),
            bind_addr: std::env::var("BOTIP_BIND_ADDR")
                .unwrap_or("127.0.0.1".to_string())
                .to_string(),
            bind_port: std::env::var("BOTIP_BIND_PORT")
                .unwrap_or_default()
                .parse::<u16>()
                .unwrap_or(8080),
            lookup_headers,
        }
    }
}
