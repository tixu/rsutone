
    #[derive(Serialize, Deserialize,Debug)]
    pub enum Status {
        CREATED,
        HANDLED,
        SOLVED,
        REJECTED
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ErrorReport {
        pub id: Option<u32>,    
        pub context: Context,
        pub error: Error,
        pub status: Status,  
    }



    #[derive(Serialize, Deserialize,Debug)]
    pub struct Context {
        pub application_name: String,
        pub application_version : String,
        pub general_info : String,
    }

    #[derive(Serialize, Deserialize,Debug)]
    pub struct Error {
        pub error: String,
    }

    pub fn build_error_report (app_name: &str, status : Status) -> ErrorReport{
        let ctx = Context{
            application_name: String::from(app_name),
            application_version: String::from("1.0"),
            general_info: String::from("eee"),
        };

        let err = Error{
                error : String::from("error message"),
        };
        ErrorReport{id: Some(1),
                     error : err,
                     context : ctx,
                     status: status,
                    }
    }

