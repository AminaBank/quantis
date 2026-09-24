use quantis::Quantis;

pub fn main() {
    let vers = Quantis::GetLibVersion();
    println!("Quantis library version: {}", vers);

    let idq = Quantis::create().unwrap();

    let ser = idq.GetSerialNumber().unwrap();
    println!("Quantis device serial number: {}", ser);

    let mstat = idq.GetModulesStatus().unwrap();
    println!("Quantis Module status: {:?}", mstat);

    let mut buf = [0_u8; 32];
    idq.Read(&mut buf).unwrap();
    println!("Random buffer: {:?}", buf);
}
