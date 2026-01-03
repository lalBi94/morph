use std::net::UdpSocket;

const PLAYER_INPUT_PHYSICAL_EVENT_NAME: &str = "PPLIN";
const PLAYER_INPUT_NON_PHYSICAL_EVENT_NAME: &str = "NPLIN";

pub enum ClientEvent
{
    Physical(PhysicalEvent),
    NonPhysical,
    Snitch,
    Position,
    HeartBeat,
    XX
}

impl ClientEvent
{
    pub fn parse_string(token: String)
    -> Self
    {
        match token.as_str()
        {
            "MF" => Self::Physical(PhysicalEvent::MF),
            "MB" => Self::Physical(PhysicalEvent::MB),
            "ML" => Self::Physical(PhysicalEvent::ML),
            "MR" => Self::Physical(PhysicalEvent::MR),
            "JP" => Self::Physical(PhysicalEvent::JP),
            "SN" => Self::Physical(PhysicalEvent::SN),
            "ST" => Self::Snitch,
            "PO" => Self::Position,
            _ => Self::XX
        }
    }

    pub fn process(
        &self, 
        payload_string: String,
        socket: &UdpSocket
    )
    -> ()
    {
        match self
        {
            ClientEvent::Physical(physical_event) => 
            {
                let payload: String = format!("{}|{}", PLAYER_INPUT_PHYSICAL_EVENT_NAME, payload_string);
                let payload_ready: &[u8] = payload.as_bytes();
                let _ = socket.send(payload_ready);
            },
            ClientEvent::NonPhysical => {
                let payload: String = format!("{}|{}", PLAYER_INPUT_NON_PHYSICAL_EVENT_NAME, payload_string);
                let payload_ready: &[u8] = payload.as_bytes();
                let _ = socket.send(payload_ready);
            },
            ClientEvent::Snitch => {
                println!("> Snitch receive");
            },
            ClientEvent::Position => {
                println!("> Position receive");
            },
            ClientEvent::HeartBeat => {
                println!("> HeartBeat");
            },
            ClientEvent::XX => {
                println!("> Cannot process event...")
            },
        }
    }
}

pub enum PhysicalEvent 
{
    MF, MB, ML, MR,
    JP, SN, XX
}

impl PhysicalEvent
{
    pub fn to_string(&self)
    -> String
    {
        format!(
            "{}",
            match self
            {
                PhysicalEvent::MF => "MF",
                PhysicalEvent::MB => "MB",
                PhysicalEvent::ML => "ML",
                PhysicalEvent::MR => "MR",
                PhysicalEvent::JP => "JP",
                PhysicalEvent::SN => "SN",
                PhysicalEvent::XX => "XX"
            }
        )
    }

    pub fn parse_string(token: String)
    -> Self
    {
        match token.as_str()
        {
            "MF" => Self::MF,
            "MB" => Self::MB,
            "ML" => Self::ML,
            "MR" => Self::MR,
            "JP" => Self::JP,
            "SN" => Self::SN,
            _ => Self::XX
        }
    }
}
