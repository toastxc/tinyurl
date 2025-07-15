use serde::Deserialize;

use serde::Serialize;
use std::ascii::AsciiExt;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaddyFile {
    apps: Apps,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apps {
    pub http: Http,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub servers: Servers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Servers {
    pub example: Example,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub listen: Vec<String>,
    pub routes: Vec<Route>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    #[serde(rename = "match")]
    pub match_field: Vec<Match>,
    pub handle: Vec<Handle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub path: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Handle {
    pub handler: String,
    pub headers: Headers,
    #[serde(rename = "status_code")]
    pub status_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    #[serde(rename = "Location")]
    pub location: Vec<String>,
}

impl CaddyFile {
    pub fn redirects(&self) -> Vec<(&String, &String)> {
        self.apps
            .http
            .servers
            .example
            .routes
            .iter()
            .map(|a| (&a.match_field[0].path[0], &a.handle[0].headers.location[0]))
            .collect()
    }

    pub fn redirects_owned(self) -> Vec<(String, String)> {
        self.redirects()
            .into_iter()
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }
    pub fn new() -> CaddyFile {
        CaddyFile {
            apps: Apps {
                http: Http {
                    servers: Servers {
                        example: Example {
                            listen: vec![":80".to_string()],
                            routes: vec![],
                        },
                    },
                },
            },
        }
    }

    pub fn route_new(&mut self, path: impl Into<String>, location: impl Into<String>) {
        let path = path.into();
        let location = location.into();

        self.apps.http.servers.example.routes.push(Route {
            match_field: vec![Match {
                path: vec![format!("{path}")],
            }],
            handle: vec![Handle {
                handler: "static_response".to_string(),
                headers: Headers {
                    location: vec![location],
                },
                status_code: 302,
            }],
        })
    }

    pub fn caddy_validate(path: impl Into<String>, url: impl Into<String>) -> Result<(), String> {
        let path = path.into();
        let mut url = url.into();


        // path validate
        if !path.is_ascii() {
            return Err(String::from(
                "Invalid characters for URL or PATH; ASCII only.",
            ));
        }
        if path.chars().into_iter().next().unwrap() != '/' || !path.chars().count() == 6 {
            return Err(String::from("Invalid PATH variable! (needs /01234)"));
        };

        // url validate
        if let Err(error) = url::Url::parse(&*url) {
            return Err(format!("{}/n URL: {url}", error.to_string()));
        }
        Ok(())
    }

    pub fn remove(&mut self, path: impl Into<String>) {
        let path = path.into();
        for (index, (redirect, _)) in self.clone().redirects_owned().into_iter().enumerate() {
            println!("{} {}", redirect, path);
            if redirect == path {
                self.apps.http.servers.example.routes.remove(index);
            }
        }
    }

    pub fn export(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

// will return false if check failed
fn new_valid(i: impl Into<Box<[bool]>>) -> bool {
    i.into().contains(&false)
}
