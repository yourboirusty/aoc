mod day;
#[macro_use]
extern crate log;

pub use day::{Day, Solveable};

#[macro_export]
macro_rules! main_day_fn {
    ($( $parts: ident), *) => {

        use base::{Day, Solveable};

        #[macro_use]
        extern crate log;

        extern crate pretty_env_logger;

        fn main() {
            pretty_env_logger::init();
            let file_str = file!().to_string();
            let day_name = file_str
                .rsplit_once('/')
                .unwrap()
                .1
                .rsplit_once('.')
                .unwrap()
                .0;
            let root_folder = file_str.rsplit_once("src").unwrap().0;
            let path = format!("{root_folder}input/{day_name}.txt");

            let day_obj = Day::new(day_name.to_string(), vec![$(Box::new($parts)),*]);
            day_obj.solve_standalone(&path);
        }
    };
}
