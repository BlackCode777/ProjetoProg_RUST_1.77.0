// ENUMS with tuplas
enum EnderecoIP{
    V4((String, String)),
    V6(String)
}

fn mostar_ip( ip: EnderecoIP ){
    match ip {
        EnderecoIP::V4(valor) => {
            println!("Endereco IPV4: {}.{}", valor.0, valor.1);
        },
        EnderecoIP::V6(valor) => {
            println!("Endereco IPV6: {}", valor);
        }
    }
}

fn main(){

    let ip_com_ipv6 = EnderecoIP::V6(String::from("::1"));
    let ip_com_ipv4 = EnderecoIP::V4(
        ("192.168.0.1".to_string(), "Informação adicional".to_string())
    );

    mostar_ip(ip_com_ipv4);
    mostar_ip(ip_com_ipv6);

}

