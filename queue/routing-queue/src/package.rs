use std::{ net::IpAddr, time::SystemTime};

pub enum PackageParseError {
    MissingArgument(String),
    InvalidId(String),
    InvalidIpAddress(String),
    InvalidSize(String),
    InvalidFormat(String),
}

struct PackageValidator;

impl std::fmt::Display for PackageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PackageParseError::MissingArgument(field) =>
                write!(f, "Falta argumento requerido: {}", field),
            PackageParseError::InvalidId(val) =>
                write!(f, "ID de paquete inválido: {}", val),
            PackageParseError::InvalidIpAddress(val) =>
                write!(f, "Dirección IP inválida: {}", val),
            PackageParseError::InvalidSize(val) =>
                write!(f, "Tamaño de paquete inválido: {}", val),
            PackageParseError::InvalidFormat(msg) =>
                write!(f, "Formato inválido: {}", msg),
        }
    }
}

impl PackageValidator {
  fn missing_arg(field_name: &str) -> PackageParseError{
    PackageParseError::MissingArgument(format!("Argumento faltante {}", field_name))
  }

  fn id_validator(id_str: &str) -> Result<PackageId, PackageParseError>{

    let id: PackageId = id_str.trim().parse().map_err(|_| PackageParseError::InvalidId("Id no se puede convertir a numero".to_string()))?;

    if id <= 0 {
        return Err(PackageParseError::InvalidId("ID no puede ser  menor o igual a 0".to_string()));
    }

    Ok(id as PackageId)
  }

  fn validate_ip_address(ip_str: &str) -> Result<IpAddr, PackageParseError> {
        ip_str.trim().parse::<IpAddr>()
            .map_err(|_| PackageParseError::InvalidIpAddress("Direccion ip malformada".to_string()))
  }

     fn validate_package_size(size_str: &str) -> Result<PackageSize, PackageParseError> {
        let size: PackageSize = size_str.trim().parse()
            .map_err(|_| PackageParseError::InvalidSize("Tamaño de paquete no es valido".to_string()))?;

        const MAX_STANDARD_MTU: u32 = 9000;

        if size == 0 {
            return Err(PackageParseError::InvalidSize("Tamaño no puede ser 0".to_string()));
        }

        if size > MAX_STANDARD_MTU {
            return Err(PackageParseError::InvalidSize(
                format!("Tamaño {} excede MTU máximo de {}", size, MAX_STANDARD_MTU)
            ));
        }

        Ok(size)
    }
}

type PackageId = u32;
type PackageSize = u32;

pub struct Package {
  pub id: PackageId,
  pub source_ip: IpAddr,
  pub destine_ip: IpAddr,
  pub size: PackageSize,
  pub timestamp: SystemTime
}

impl Package {

  pub fn build(args: &[&str]) -> Result<Self, PackageParseError> {

     let id_str = args.get(0).ok_or_else(|| PackageValidator::missing_arg("id_arg"))?;
     let src_ip_str = args.get(1).ok_or_else(|| PackageValidator::missing_arg("src_ip_arg"))?;
     let dest_ip_str = args.get(2).ok_or_else(|| PackageValidator::missing_arg("dest_ip_arg"))?;
     let s_bytes_str = args.get(3).ok_or_else(|| PackageValidator::missing_arg("s_bytes_arg"))?;

     let id = PackageValidator::id_validator(&id_str)?;
     let source_ip = PackageValidator::validate_ip_address(&src_ip_str)?;
     let destine_ip = PackageValidator::validate_ip_address(&dest_ip_str)?;
     let size = PackageValidator::validate_package_size(&s_bytes_str)?;

     if source_ip == destine_ip {
        return Err(PackageParseError::InvalidFormat(
            "IP origen y destino no pueden ser iguales".to_string()
        ));
     }

    Ok(Self {
      id,
      destine_ip,
      size,
      source_ip,
      timestamp: SystemTime::now()
    })
  }
}
