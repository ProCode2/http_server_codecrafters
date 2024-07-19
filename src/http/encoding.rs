#[derive(Debug, Clone)]
pub enum EncodingType {
    Gzip(f32),
}

impl EncodingType {
    fn from_string(s: &str, q: f32) -> Option<EncodingType> {
        match s {
            "gzip" => Some(Self::Gzip(q)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Encoding {
    encoding_type: EncodingType,
}

impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.encoding_type {
            EncodingType::Gzip(_) => write!(f, "gzip"),
        }
    }
}

impl Encoding {
    pub fn get_endoing_scheme(s: &str) -> Option<Encoding> {
        // gzip;q=1.0, deflate
        let encodings = s.split(",").take_while(|x| !x.is_empty()).map(|x| x.trim());
        let encodings = encodings
            .map(|enc| {
                let enc_type = if let Some((enc_type, q)) = enc.split_once(";") {
                    let val = if let Some((_, value)) = q.split_once("=") {
                        value.parse::<f32>().unwrap_or(1.0)
                    } else {
                        1.0
                    };
                    EncodingType::from_string(enc_type, val)
                } else {
                    EncodingType::from_string(enc, 1.0)
                };

                if let Some(t) = enc_type {
                    Some(Encoding { encoding_type: t })
                } else {
                    None
                }
            })
            .take_while(|x| x.is_some())
            .collect::<Vec<Option<Encoding>>>();
        let result = encodings.first();
        match result {
            Some(x) => x.clone(),
            None => None,
        }
    }
}
