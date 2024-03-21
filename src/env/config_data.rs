pub struct ConfigData
{
    args: Vec<String>,
    mem_size: u16,
    starting_pc: u16,
}

impl ConfigData
{
    pub fn new(args: Vec<String>) -> Self
    {
        Self { args, starting_pc: 0, mem_size: 0 }
    }

    #[allow(dead_code)]
    pub fn get_mem_size(&self) -> u16
    {
        self.mem_size
    }

    #[allow(dead_code)]
    pub fn get_starting_pc(&self) -> u16
    {
        self.starting_pc
    }

    pub fn parse(&mut self) -> Option<(i32, String)>
    {
        let arg_count = self.args.len();
        println!("[TEST]: arg count is {0}", arg_count);

        if arg_count == 0
        {
            return Some((-1, String::from("No input args to parse")));
        }

        let mut skip_next = false;

        for i in 1..arg_count
        {
            if skip_next
            {
                skip_next = false;
                continue;
            }

            let arg = self.args.get(i).unwrap();

            if arg == "--mem-size"
            {
                let opt_next_arg = self.args.get(i + 1);

                if opt_next_arg.is_none()
                {
                    return Some((-2, String::from("Expected another arg after '--pc'")));
                }

                let val_result = opt_next_arg.unwrap().parse::<u16>();

                match val_result
                {
                    Ok(val) => { self.mem_size = val; },
                    Err(e) => { return Some((-2, e.to_string())); }
                }

                skip_next = true;
            }

            else if arg == "--pc"
            {
                let opt_next_arg = self.args.get(i + 1);

                if opt_next_arg.is_none()
                {
                    return Some((-2, String::from("Expected another arg after '--pc'")));
                }

                let val_result = opt_next_arg.unwrap().parse::<u16>();

                match val_result
                {
                    Ok(val) => { self.starting_pc = val; },
                    Err(e) => { return Some((-2, e.to_string())); }
                }

                skip_next = true;
            }

            else
            {
                // TODO: Include the arg in the error message??
                return Some((-3, String::from("Unexpected arg")));
            }
        }

        // Some((-3, String::from("Failed to parse input args")))
        return None;
    }
}

#[cfg(test)]
mod tests
{
    use crate::env::config_data::ConfigData;

    #[test]
    fn parse_no_args_fails()
    {
        let args = Vec::<String>::new();
        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_some());
        assert_ne!(opt_error.unwrap().0, 0);
    }

    #[test]
    fn parse_starting_pc_valid()
    {
        let mut args = Vec::<String>::new();
        args.push(String::from("exe"));
        args.push(String::from("--pc"));
        args.push(String::from("512"));

        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_none());
        assert_eq!(config_data.get_starting_pc(), 512);
    }

    #[test]
    fn parse_starting_pc_missing_value()
    {
        let mut args = Vec::<String>::new();
        args.push(String::from("exe"));
        args.push(String::from("--pc"));

        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_some());
        assert_ne!(opt_error.unwrap().0, 0);
    }

    #[test]
    fn parse_starting_pc_bad_value()
    {
        let mut args = Vec::<String>::new();
        args.push(String::from("exe"));
        args.push(String::from("--pc"));
        args.push(String::from("-512"));

        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_some());
    }

    #[test]
    fn parse_mem_size_valid()
    {
        let mut args = Vec::<String>::new();
        args.push(String::from("exe"));
        args.push(String::from("--mem-size"));
        args.push(String::from("4096"));

        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_none());
        assert_eq!(config_data.get_mem_size(), 4096);
    }

    #[test]
    fn parse_pc_and_mem_size_valid()
    {
        let mut args = Vec::<String>::new();
        args.push(String::from("exe"));
        args.push(String::from("--pc"));
        args.push(String::from("512"));
        args.push(String::from("--mem-size"));
        args.push(String::from("4096"));

        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_none());
        assert_eq!(config_data.get_starting_pc(), 512);
        assert_eq!(config_data.get_mem_size(), 4096);
    }

    #[test]
    fn parse_pc_and_mem_size_order_flipped_valid()
    {
        let mut args = Vec::<String>::new();
        args.push(String::from("exe"));
        args.push(String::from("--mem-size"));
        args.push(String::from("4096"));
        args.push(String::from("--pc"));
        args.push(String::from("512"));

        let mut config_data = ConfigData::new(args.clone());
        let opt_error = config_data.parse();

        assert!(opt_error.is_none());
        assert_eq!(config_data.get_starting_pc(), 512);
        assert_eq!(config_data.get_mem_size(), 4096);
    }
}

