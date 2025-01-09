pub mod myvec;
pub mod myhashmap;
pub mod myhashset;
pub mod myiterations;
pub mod mydatetime;

pub mod mythreads;
pub mod myscopedthreads;
pub mod my_mutex;
pub mod mympsc;
pub mod myfs;
pub mod myusrinpt;

fn main() {
    // myvec::test_vec_car();
    // myhashmap::test_hashmap_basic();
    // myhashset::test_hashset_basic();
    // myiterations::test_iters();
    // mydatetime::test_stdtime();
    // mydatetime::test_chrono();
    // mythreads::test_threads();
    // mythreads::spawn_threads();
    // myscopedthreads::test_threads_variables();
    // my_mutex::test_mutex();
    // mympsc::test_channels();
    // myfs::test_create_dir();
    // myfs::create_files();
    // myfs::remove_dir();
    // myfs::read_file();
    // println!("Generated uuid is {}", uuid::Uuid::new_v4().to_string());

    myusrinpt::get_user_input();

    
}