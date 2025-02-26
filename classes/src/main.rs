struct Computer {
    brand: String,
    cpu: String,
    gpu: String,
    ram: i32,
    storage: String,

}
impl Computer {
    fn new(brand: String, cpu: String, gpu: String, ram: i32, storage: String) -> Computer {
        Computer {
            brand,
            cpu,
            gpu,
            ram,
            storage,
        }
    }
    fn print(&self) {
        println!("My computer is a {} with a {} CPU, {} GPU, {}GB of RAM, and {} storage", self.brand, self.cpu, self.gpu, self.ram, self.storage);
    }
}
fn main() {
    let my_comp = Computer {
        brand: String::from("Dell"),
        cpu: String::from("Intel i7"),
        gpu: String::from("Nvidia RTX 2080"),
        ram: 16,
        storage: String::from("1TB SSD"),
    };
    println!("My computer is a {} with a {} CPU, {} GPU, {}GB of RAM, and {} storage", my_comp.brand, my_comp.cpu, my_comp.gpu, my_comp.ram, my_comp.storage);
}
