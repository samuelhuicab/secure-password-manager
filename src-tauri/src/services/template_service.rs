use uuid::Uuid;

use crate::models::{
    field::Field,
    item_type::ItemType,
};

pub struct TemplateService;

impl TemplateService {

    fn field(
        key:&str,
        label:&str,
        hidden:bool,
    )->Field{

        Field{

            id:Uuid::new_v4().to_string(),

            key:key.to_string(),

            label:label.to_string(),

            value:String::new(),

            hidden,

        }

    }

    pub fn create_fields(
        item_type:&ItemType,
    )->Vec<Field>{

        match item_type{

            ItemType::Server=>vec![

                Self::field("host","Host",false),

                Self::field("port","Puerto",false),

                Self::field("username","Usuario",false),

                Self::field("password","Contraseña",true),

                Self::field("private_key","Clave privada",true),

            ],

            ItemType::Database=>vec![

                Self::field("host","Host",false),

                Self::field("port","Puerto",false),

                Self::field("database","Base de datos",false),

                Self::field("username","Usuario",false),

                Self::field("password","Contraseña",true),

            ],

            ItemType::Email=>vec![

                Self::field("email","Correo",false),

                Self::field("password","Contraseña",true),

                Self::field("smtp","SMTP",false),

                Self::field("imap","IMAP",false),

            ],

            ItemType::Website=>vec![

                Self::field("url","URL",false),

                Self::field("username","Usuario",false),

                Self::field("password","Contraseña",true),

            ],

            ItemType::Api=>vec![

                Self::field("endpoint","Endpoint",false),

                Self::field("token","Token",true),

                Self::field("secret","Secret",true),

            ],

            ItemType::License=>vec![

                Self::field("product","Producto",false),

                Self::field("license","Licencia",true),

            ],

            ItemType::Note=>vec![

                Self::field("content","Contenido",false),

            ],

            // Las variables de entorno se añaden a mano (o importando un .env).
            ItemType::Env=>vec![],

        }

    }

}