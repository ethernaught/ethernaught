
/*
pub const LAYER_ORDER: [&str; 6] = [
    "ethernet",
    "sll2",
    "ipv4",
    "ipv6",
    "arp",
    "udp",
    "tcp",
    "dhcp"
];
*/

#[derive(Debug, Clone)]
pub struct QueryField {
    pub name: String,
    pub value: String
}

#[derive(Debug, Clone)]
pub struct PacketQuery {
    pub layer: String,
    pub field: Option<QueryField>
}

impl PacketQuery {

    pub fn from(query: &str) -> Vec<Self> {
        let mut res = Vec::new();
        let parts = query.split_whitespace().collect::<Vec<_>>();

        let mut is_and = true;

        for part in parts {
            match part {
                "&" => is_and = true,
                "|" => is_and = false,
                _ => {
                    if part.contains('=') {
                        let split: Vec<&str> = part.splitn(2, '=').collect();
                        let (key, value) = (split[0], split[1].trim_matches('"'));

                        if let Some((layer, field)) = key.rsplit_once('.') {
                            res.push(PacketQuery {
                                layer: layer.to_string(),
                                field: Some(QueryField {
                                    name: field.to_string(),
                                    value: value.to_string()
                                })
                            });
                        }
                    } else {
                        res.push(PacketQuery {
                            layer: part.to_string(),
                            field: None
                        });
                    }
                }
            }
        }
        res
    }
}
