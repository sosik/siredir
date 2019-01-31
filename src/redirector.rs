use log;
use regex::Regex;

use yaps_hyper_router as router;

pub struct Redirector {
    redirects: Vec<(Regex, String, u16)>,
}

impl Redirector {
    pub fn new() -> Self {
        Redirector { redirects: vec![] }
    }

    // adds redirection pair into list of know redirections
    pub fn add_redirection(
        &mut self,
        regex_str: impl AsRef<str>,
        replacement: impl Into<String>,
        code: u16,
    ) -> Result<(), Box<std::error::Error>> {
        Ok(self
            .redirects
            .push((Regex::new(regex_str.as_ref())?, replacement.into(), code)))
    }
}

impl router::Handler for Redirector {
    fn handle(&self, req: router::Request) -> router::HandlerResult {
        log::trace!("handling started...");

        let mut host_str = "";
        let hyper_request = req.get_hyper_request();

        if let Some(h) = hyper_request.headers().get(hyper::header::HOST) {
            host_str = h.to_str().unwrap_or("");
        }

        let full_url = format!("{}{}", host_str, hyper_request.uri().to_string());

        // find first match
        if let Some(found) = self.redirects.iter().find(|k| {
            log::trace!("Comparing {} with url {} from match", k.0, &full_url);
            k.0.is_match(&full_url)
        }) {
            return Box::new(futures::future::ok(
                hyper::Response::builder()
                    .status(found.2)
					.header("Location", found.0.replace(&full_url, found.1.as_str()).into_owned())
                    .body(hyper::Body::from(
                        found.0.replace(&full_url, found.1.as_str()).into_owned(),
                    ))
                    .unwrap(),
            ));
        } else {
            log::debug!("no match found for '{}'", &full_url);
            return Box::new(futures::future::ok(
                hyper::Response::builder()
                    .status(404)
                    .body(hyper::Body::from("Not found!"))
                    .unwrap(),
            ));
        }
    }
}
