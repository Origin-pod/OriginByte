/**
 * @file 03-Rust Design Patterns.rs
 * @brief Comprehensive Rust Design Patterns Implementation
 *
 * This file covers all major design patterns categorized into:
 * - Creational Patterns: Object creation mechanisms
 * - Structural Patterns: Object composition
 * - Behavioral Patterns: Object communication and responsibility
 *
 * Each pattern includes:
 * - Problem statement
 * - Solution approach
 * - Rust implementation leveraging language features
 * - Usage examples
 * - When to use and when not to use
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::fmt;

// ================================================================================
// CREATIONAL PATTERNS
// ================================================================================

// ================================================================================
// 1. SINGLETON PATTERN
// ================================================================================

/**
 * Problem: Ensure a type has only one instance and provide global access to it.
 * Use Case: Logger, Database connection, Configuration manager
 * Rust Approach: Use std::sync::OnceLock or lazy_static for thread-safe singletons
 */

use std::sync::OnceLock;

struct Logger {
    logs: Arc<RwLock<Vec<String>>>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            logs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn log(&self, message: &str) {
        let mut logs = self.logs.write().unwrap();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        logs.push(format!("[{}] {}", timestamp, message));
    }

    fn print_logs(&self) {
        let logs = self.logs.read().unwrap();
        for log in logs.iter() {
            println!("{}", log);
        }
    }
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn get_logger() -> &'static Logger {
    LOGGER.get_or_init(|| Logger::new())
}

fn demonstrate_singleton() {
    println!("=== SINGLETON PATTERN ===");

    let logger = get_logger();
    logger.log("Application started");
    logger.log("User logged in");
    logger.log("Processing data");

    logger.print_logs();
    println!();
}

// ================================================================================
// 2. FACTORY METHOD PATTERN
// ================================================================================

/**
 * Problem: Create objects without specifying the exact type.
 * Use Case: Frameworks, libraries, plugin systems
 * Rust Approach: Use traits and enums for type-safe object creation
 */

trait Animal {
    fn make_sound(&self);
    fn get_type(&self) -> &str;
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }

    fn get_type(&self) -> &str {
        "Dog"
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }

    fn get_type(&self) -> &str {
        "Cat"
    }
}

struct Cow;
impl Animal for Cow {
    fn make_sound(&self) {
        println!("Moo!");
    }

    fn get_type(&self) -> &str {
        "Cow"
    }
}

enum AnimalType {
    Dog,
    Cat,
    Cow,
}

struct AnimalFactory;

impl AnimalFactory {
    fn create_animal(animal_type: AnimalType) -> Box<dyn Animal> {
        match animal_type {
            AnimalType::Dog => Box::new(Dog),
            AnimalType::Cat => Box::new(Cat),
            AnimalType::Cow => Box::new(Cow),
        }
    }

    fn create_random_animal() -> Box<dyn Animal> {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        match count % 3 {
            0 => Box::new(Dog),
            1 => Box::new(Cat),
            _ => Box::new(Cow),
        }
    }
}

fn demonstrate_factory_method() {
    println!("=== FACTORY METHOD PATTERN ===");

    let animal_types = vec![AnimalType::Dog, AnimalType::Cat, AnimalType::Cow];

    for animal_type in animal_types {
        let animal = AnimalFactory::create_animal(animal_type);
        println!("Created a {}: ", animal.get_type());
        animal.make_sound();
    }

    println!("Random animals:");
    for _ in 0..3 {
        let animal = AnimalFactory::create_random_animal();
        println!("Created a {}: ", animal.get_type());
        animal.make_sound();
    }
    println!();
}

// ================================================================================
// 3. ABSTRACT FACTORY PATTERN
// ================================================================================

/**
 * Problem: Create families of related objects without specifying concrete classes.
 * Use Case: UI themes, database connectors, cross-platform development
 * Rust Approach: Use traits for factories and implement for different families
 */

trait Button {
    fn paint(&self);
}

trait Checkbox {
    fn paint(&self);
}

trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

// Windows implementations
struct WindowsButton;
impl Button for WindowsButton {
    fn paint(&self) {
        println!("Windows Button rendered");
    }
}

struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn paint(&self) {
        println!("Windows Checkbox rendered");
    }
}

struct WindowsFactory;
impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}

// macOS implementations
struct MacOSButton;
impl Button for MacOSButton {
    fn paint(&self) {
        println!("macOS Button rendered");
    }
}

struct MacOSCheckbox;
impl Checkbox for MacOSCheckbox {
    fn paint(&self) {
        println!("macOS Checkbox rendered");
    }
}

struct MacOSFactory;
impl GUIFactory for MacOSFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacOSButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacOSCheckbox)
    }
}

fn render_gui(factory: &dyn GUIFactory) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();

    button.paint();
    checkbox.paint();
}

fn demonstrate_abstract_factory() {
    println!("=== ABSTRACT FACTORY PATTERN ===");

    println!("Windows GUI:");
    let windows_factory = WindowsFactory;
    render_gui(&windows_factory);

    println!("\nmacOS GUI:");
    let mac_factory = MacOSFactory;
    render_gui(&mac_factory);
    println!();
}

// ================================================================================
// 4. BUILDER PATTERN
// ================================================================================

/**
 * Problem: Construct complex objects step by step.
 * Use Case: Complex object construction, configuration objects
 * Rust Approach: Use method chaining and type-level builders
 */

#[derive(Debug, Default)]
struct Computer {
    cpu: String,
    gpu: String,
    ram: u32,
    storage: u32,
    has_wifi: bool,
    has_bluetooth: bool,
}

impl Computer {
    fn display(&self) {
        println!("Computer Configuration:");
        println!("  CPU: {}", self.cpu);
        println!("  GPU: {}", self.gpu);
        println!("  RAM: {}GB", self.ram);
        println!("  Storage: {}GB", self.storage);
        println!("  WiFi: {}", if self.has_wifi { "Yes" } else { "No" });
        println!("  Bluetooth: {}", if self.has_bluetooth { "Yes" } else { "No" });
    }
}

struct ComputerBuilder {
    computer: Computer,
}

impl ComputerBuilder {
    fn new() -> Self {
        ComputerBuilder {
            computer: Computer::default(),
        }
    }

    fn cpu(mut self, cpu: &str) -> Self {
        self.computer.cpu = cpu.to_string();
        self
    }

    fn gpu(mut self, gpu: &str) -> Self {
        self.computer.gpu = gpu.to_string();
        self
    }

    fn ram(mut self, ram: u32) -> Self {
        self.computer.ram = ram;
        self
    }

    fn storage(mut self, storage: u32) -> Self {
        self.computer.storage = storage;
        self
    }

    fn wifi(mut self, has_wifi: bool) -> Self {
        self.computer.has_wifi = has_wifi;
        self
    }

    fn bluetooth(mut self, has_bluetooth: bool) -> Self {
        self.computer.has_bluetooth = has_bluetooth;
        self
    }

    fn build(self) -> Computer {
        self.computer
    }
}

impl Default for ComputerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn demonstrate_builder() {
    println!("=== BUILDER PATTERN ===");

    println!("Building Gaming Computer:");
    let gaming_pc = ComputerBuilder::new()
        .cpu("Intel Core i9-12900K")
        .gpu("NVIDIA RTX 4090")
        .ram(32)
        .storage(2000)
        .wifi(true)
        .bluetooth(true)
        .build();
    gaming_pc.display();

    println!("\nBuilding Office Computer:");
    let office_pc = ComputerBuilder::new()
        .cpu("Intel Core i5-12400")
        .gpu("Intel UHD Graphics 730")
        .ram(16)
        .storage(512)
        .wifi(true)
        .bluetooth(false)
        .build();
    office_pc.display();
    println!();
}

// ================================================================================
// 5. PROTOTYPE PATTERN
// ================================================================================

/**
 * Problem: Create new objects by copying existing objects.
 * Use Case: When object creation is expensive, configuration management
 * Rust Approach: Use Clone trait for copying objects
 */

trait Shape: Clone {
    fn draw(&self);
    fn set_position(&mut self, x: i32, y: i32);
    fn get_position(&self) -> (i32, i32);
}

#[derive(Clone)]
struct Circle {
    radius: i32,
    x: i32,
    y: i32,
}

impl Circle {
    fn new(radius: i32, x: i32, y: i32) -> Self {
        Circle { radius, x, y }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing Circle at ({}, {}) with radius {}", self.x, self.y, self.radius);
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Clone)]
struct Rectangle {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32, x: i32, y: i32) -> Self {
        Rectangle { width, height, x, y }
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing Rectangle at ({}, {}) with size {}x{}",
                 self.x, self.y, self.width, self.height);
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

struct ShapePrototypeManager {
    prototypes: HashMap<String, Box<dyn Shape>>,
}

impl ShapePrototypeManager {
    fn new() -> Self {
        ShapePrototypeManager {
            prototypes: HashMap::new(),
        }
    }

    fn add_prototype(&mut self, name: &str, prototype: Box<dyn Shape>) {
        self.prototypes.insert(name.to_string(), prototype);
    }

    fn create_shape(&self, name: &str) -> Option<Box<dyn Shape>> {
        self.prototypes.get(name).map(|shape| shape.clone_box())
    }
}

// Helper trait for cloning trait objects
trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T: Shape + Clone + 'static> ShapeClone for T {
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}

fn demonstrate_prototype() {
    println!("=== PROTOTYPE PATTERN ===");

    let mut manager = ShapePrototypeManager::new();

    // Add prototypes
    manager.add_prototype("small_circle", Box::new(Circle::new(5, 0, 0)));
    manager.add_prototype("large_circle", Box::new(Circle::new(20, 0, 0)));
    manager.add_prototype("small_rectangle", Box::new(Rectangle::new(10, 5, 0, 0)));

    // Create shapes from prototypes
    let mut circle1 = manager.create_shape("small_circle").unwrap();
    let mut circle2 = manager.create_shape("small_circle").unwrap();
    let mut large_circle = manager.create_shape("large_circle").unwrap();
    let mut rectangle = manager.create_shape("small_rectangle").unwrap();

    // Position and draw shapes
    circle1.set_position(10, 10);
    circle2.set_position(50, 50);
    large_circle.set_position(100, 100);
    rectangle.set_position(30, 30);

    circle1.draw();
    circle2.draw();
    large_circle.draw();
    rectangle.draw();
    println!();
}

// ================================================================================
// STRUCTURAL PATTERNS
// ================================================================================

// ================================================================================
// 6. ADAPTER PATTERN
// ================================================================================

/**
 * Problem: Allow incompatible interfaces to work together.
 * Use Case: Third-party libraries, legacy systems, API integration
 * Rust Approach: Use traits to define target interface and adapt existing types
 */

trait MediaPlayer {
    fn play(&self, audio_type: &str, file_name: &str);
}

trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: &str);
    fn play_mp4(&self, file_name: &str);
}

struct VlcPlayer;
impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: &str) {
        println!("Playing vlc file: {}", file_name);
    }

    fn play_mp4(&self, _file_name: &str) {
        // Do nothing
    }
}

struct Mp4Player;
impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: &str) {
        // Do nothing
    }

    fn play_mp4(&self, file_name: &str) {
        println!("Playing mp4 file: {}", file_name);
    }
}

struct MediaAdapter {
    advanced_player: Box<dyn AdvancedMediaPlayer>,
}

impl MediaAdapter {
    fn new(audio_type: &str) -> Self {
        match audio_type {
            "vlc" => MediaAdapter {
                advanced_player: Box::new(VlcPlayer),
            },
            "mp4" => MediaAdapter {
                advanced_player: Box::new(Mp4Player),
            },
            _ => panic!("Unsupported audio type: {}", audio_type),
        }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, audio_type: &str, file_name: &str) {
        match audio_type {
            "vlc" => self.advanced_player.play_vlc(file_name),
            "mp4" => self.advanced_player.play_mp4(file_name),
            _ => {}
        }
    }
}

struct AudioPlayer {
    media_adapter: Option<MediaAdapter>,
}

impl AudioPlayer {
    fn new() -> Self {
        AudioPlayer {
            media_adapter: None,
        }
    }
}

impl MediaPlayer for AudioPlayer {
    fn play(&self, audio_type: &str, file_name: &str) {
        match audio_type {
            "mp3" => println!("Playing mp3 file: {}", file_name),
            "vlc" | "mp4" => {
                let adapter = MediaAdapter::new(audio_type);
                adapter.play(audio_type, file_name);
            }
            _ => println!("Invalid media. {} format not supported", audio_type),
        }
    }
}

fn demonstrate_adapter() {
    println!("=== ADAPTER PATTERN ===");

    let player = AudioPlayer::new();

    player.play("mp3", "song.mp3");
    player.play("mp4", "video.mp4");
    player.play("vlc", "movie.vlc");
    player.play("avi", "movie.avi");
    println!();
}

// ================================================================================
// 7. DECORATOR PATTERN
// ================================================================================

/**
 * Problem: Add new functionality to objects dynamically without altering their structure.
 * Use Case: UI components, I/O streams, feature toggles
 * Rust Approach: Use trait objects and wrapper structs
 */

trait Coffee {
    fn get_cost(&self) -> f64;
    fn get_description(&self) -> &str;
}

struct SimpleCoffee;
impl Coffee for SimpleCoffee {
    fn get_cost(&self) -> f64 {
        2.0
    }

    fn get_description(&self) -> &str {
        "Simple Coffee"
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn get_cost(&self) -> f64 {
        self.coffee.get_cost() + 0.5
    }

    fn get_description(&self) -> &str {
        // This is a limitation of Rust trait design
        // In practice, you'd use String returns or more complex patterns
        "Coffee with Milk"
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}

impl Coffee for SugarDecorator {
    fn get_cost(&self) -> f64 {
        self.coffee.get_cost() + 0.2
    }

    fn get_description(&self) -> &str {
        "Coffee with Sugar"
    }
}

// Alternative approach using composition and newtypes
#[derive(Debug)]
struct CoffeeCup {
    description: String,
    cost: f64,
}

impl CoffeeCup {
    fn new() -> Self {
        CoffeeCup {
            description: "Simple Coffee".to_string(),
            cost: 2.0,
        }
    }

    fn with_milk(mut self) -> Self {
        self.description += ", Milk";
        self.cost += 0.5;
        self
    }

    fn with_sugar(mut self) -> Self {
        self.description += ", Sugar";
        self.cost += 0.2;
    }

    fn with_whipped_cream(mut self) -> Self {
        self.description += ", Whipped Cream";
        self.cost += 1.0;
    }

    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

fn demonstrate_decorator() {
    println!("=== DECORATOR PATTERN ===");

    // Using traditional decorator pattern (limited by Rust's trait system)
    println!("Traditional decorator approach:");
    let coffee = Box::new(SimpleCoffee);
    println!("{} ${:.2}", coffee.get_description(), coffee.get_cost());

    let coffee = Box::new(MilkDecorator::new(coffee));
    println!("{} ${:.2}", coffee.get_description(), coffee.get_cost());

    // Using builder-style decorator pattern (more idiomatic in Rust)
    println!("\nIdiomatic Rust approach:");
    let coffee = CoffeeCup::new();
    println!("{} ${:.2}", coffee.get_description(), coffee.get_cost());

    let coffee = coffee.with_milk();
    println!("{} ${:.2}", coffee.get_description(), coffee.get_cost());

    let coffee = coffee.with_sugar();
    println!("{} ${:.2}", coffee.get_description(), coffee.get_cost());

    let coffee = coffee.with_whipped_cream();
    println!("{} ${:.2}", coffee.get_description(), coffee.get_cost());
    println!();
}

// ================================================================================
// 8. FACADE PATTERN
// ================================================================================

/**
 * Problem: Provide a simplified interface to a complex subsystem.
 * Use Case: Complex libraries, API simplification, legacy systems
 * Rust Approach: Use structs to encapsulate complex subsystem interactions
 */

struct CPU;
impl CPU {
    fn freeze(&self) {
        println!("CPU: Freezing processor");
    }

    fn jump(&self, position: u64) {
        println!("CPU: Jumping to position {}", position);
    }

    fn execute(&self) {
        println!("CPU: Executing instructions");
    }
}

struct Memory;
impl Memory {
    fn load(&self, position: u64, data: &[u8]) {
        println!("Memory: Loading {} bytes at position {}", data.len(), position);
    }
}

struct HardDrive;
impl HardDrive {
    fn read(&self, lba: u64, size: usize) -> Vec<u8> {
        println!("Hard Drive: Reading {} bytes from LBA {}", size, lba);
        vec![0; size] // Simulate reading data
    }
}

struct GPU;
impl GPU {
    fn render(&self) {
        println!("GPU: Rendering graphics");
    }
}

struct SoundCard;
impl SoundCard {
    fn play_sound(&self) {
        println!("Sound Card: Playing startup sound");
    }
}

struct ComputerFacade {
    cpu: CPU,
    memory: Memory,
    hard_drive: HardDrive,
    gpu: GPU,
    sound_card: SoundCard,
}

impl ComputerFacade {
    fn new() -> Self {
        ComputerFacade {
            cpu: CPU,
            memory: Memory,
            hard_drive: HardDrive,
            gpu: GPU,
            sound_card: SoundCard,
        }
    }

    fn start(&self) {
        println!("Starting computer...");

        self.cpu.freeze();

        let boot_data = self.hard_drive.read(0, 1024);
        self.memory.load(0, &boot_data);

        self.cpu.jump(0);
        self.cpu.execute();

        self.gpu.render();
        self.sound_card.play_sound();

        println!("Computer started successfully!");
    }

    fn shutdown(&self) {
        println!("Shutting down computer...");
        println!("Computer shut down successfully!");
    }
}

fn demonstrate_facade() {
    println!("=== FACADE PATTERN ===");

    let computer = ComputerFacade::new();
    computer.start();
    println!();
    computer.shutdown();
    println!();
}

// ================================================================================
// 9. FLYWEIGHT PATTERN
// ================================================================================

/**
 * Problem: Minimize memory usage by sharing as much data as possible.
 * Use Case: Text editors, game development, graphics rendering
 * Rust Approach: Use Arc for shared data and struct for context-specific data
 */

#[derive(Debug, Eq, PartialEq, Hash)]
struct TreeTypeKey {
    name: String,
    color: String,
    texture: String,
}

struct TreeType {
    name: String,
    color: String,
    texture: String,
}

impl TreeType {
    fn new(name: &str, color: &str, texture: &str) -> Self {
        TreeType {
            name: name.to_string(),
            color: color.to_string(),
            texture: texture.to_string(),
        }
    }

    fn draw(&self, x: i32, y: i32) {
        println!("Drawing {} {} tree at ({}, {}) with {} texture",
                 self.color, self.name, x, y, self.texture);
    }
}

struct TreeTypeFactory {
    tree_types: Arc<RwLock<HashMap<TreeTypeKey, Arc<TreeType>>>>,
}

impl TreeTypeFactory {
    fn new() -> Self {
        TreeTypeFactory {
            tree_types: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_tree_type(&self, name: &str, color: &str, texture: &str) -> Arc<TreeType> {
        let key = TreeTypeKey {
            name: name.to_string(),
            color: color.to_string(),
            texture: texture.to_string(),
        };

        let mut tree_types = self.tree_types.write().unwrap();
        tree_types.entry(key).or_insert_with(|| {
            Arc::new(TreeType::new(name, color, texture))
        }).clone()
    }

    fn get_tree_type_count(&self) -> usize {
        self.tree_types.read().unwrap().len()
    }
}

struct Tree {
    x: i32,
    y: i32,
    tree_type: Arc<TreeType>,
}

impl Tree {
    fn new(x: i32, y: i32, tree_type: Arc<TreeType>) -> Self {
        Tree { x, y, tree_type }
    }

    fn draw(&self) {
        self.tree_type.draw(self.x, self.y);
    }
}

struct Forest {
    trees: Vec<Tree>,
    tree_type_factory: TreeTypeFactory,
}

impl Forest {
    fn new() -> Self {
        Forest {
            trees: Vec::new(),
            tree_type_factory: TreeTypeFactory::new(),
        }
    }

    fn plant_tree(&mut self, x: i32, y: i32, name: &str, color: &str, texture: &str) {
        let tree_type = self.tree_type_factory.get_tree_type(name, color, texture);
        self.trees.push(Tree::new(x, y, tree_type));
    }

    fn draw(&self) {
        for tree in &self.trees {
            tree.draw();
        }
    }

    fn print_statistics(&self) {
        println!("Total trees: {}", self.trees.len());
        println!("Unique tree types: {}", self.tree_type_factory.get_tree_type_count());
        println!("Memory saved by sharing: {} objects",
                 self.trees.len() - self.tree_type_factory.get_tree_type_count());
    }
}

fn demonstrate_flyweight() {
    println!("=== FLYWEIGHT PATTERN ===");

    let mut forest = Forest::new();

    // Plant many trees with shared types
    forest.plant_tree(1, 1, "Oak", "Green", "Rough");
    forest.plant_tree(2, 3, "Oak", "Green", "Rough");
    forest.plant_tree(5, 4, "Oak", "Green", "Rough");
    forest.plant_tree(3, 6, "Oak", "Green", "Rough");

    forest.plant_tree(2, 1, "Pine", "Dark Green", "Smooth");
    forest.plant_tree(4, 3, "Pine", "Dark Green", "Smooth");
    forest.plant_tree(6, 5, "Pine", "Dark Green", "Smooth");

    forest.plant_tree(1, 5, "Maple", "Red", "Smooth");
    forest.plant_tree(3, 3, "Maple", "Red", "Smooth");

    println!("Drawing forest:");
    forest.draw();

    println!("\nMemory statistics:");
    forest.print_statistics();
    println!();
}

// ================================================================================
// 10. PROXY PATTERN
// ================================================================================

/**
 * Problem: Provide a surrogate or placeholder for another object to control access.
 * Use Case: Lazy loading, access control, caching, logging
 * Rust Approach: Use structs to wrap and delegate to the real object
 */

trait Image {
    fn display(&self);
}

struct RealImage {
    filename: String,
}

impl RealImage {
    fn new(filename: &str) -> Self {
        println!("Loading image {} from disk...", filename);
        // Simulate loading delay
        thread::sleep(Duration::from_millis(100));
        RealImage {
            filename: filename.to_string(),
        }
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("Displaying image {}", self.filename);
    }
}

struct ProxyImage {
    filename: String,
    real_image: Option<RealImage>,
}

impl ProxyImage {
    fn new(filename: &str) -> Self {
        ProxyImage {
            filename: filename.to_string(),
            real_image: None,
        }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        // Note: This is a simplified example
        // In a real implementation, you'd need interior mutability
        println!("Displaying image {} (via proxy)", self.filename);
        if self.real_image.is_none() {
            let real_image = RealImage::new(&self.filename);
            real_image.display();
        }
    }
}

// More sophisticated proxy using interior mutability
struct CachedImage {
    filename: String,
    real_image: RefCell<Option<RealImage>>,
}

impl CachedImage {
    fn new(filename: &str) -> Self {
        CachedImage {
            filename: filename.to_string(),
            real_image: RefCell::new(None),
        }
    }
}

impl Image for CachedImage {
    fn display(&self) {
        let mut real_image = self.real_image.borrow_mut();
        if real_image.is_none() {
            *real_image = Some(RealImage::new(&self.filename));
        }
        real_image.as_ref().unwrap().display();
    }
}

struct ImageGallery {
    images: Vec<Box<dyn Image>>,
}

impl ImageGallery {
    fn new() -> Self {
        ImageGallery {
            images: Vec::new(),
        }
    }

    fn add_image(&mut self, filename: &str) {
        self.images.push(Box::new(CachedImage::new(filename)));
    }

    fn display_image(&self, index: usize) {
        if let Some(image) = self.images.get(index) {
            image.display();
        }
    }

    fn display_all(&self) {
        for (i, image) in self.images.iter().enumerate() {
            println!("Image {}: ", i + 1);
            image.display();
        }
    }
}

fn demonstrate_proxy() {
    println!("=== PROXY PATTERN ===");

    let mut gallery = ImageGallery::new();
    gallery.add_image("photo1.jpg");
    gallery.add_image("photo2.jpg");
    gallery.add_image("photo3.jpg");

    println!("Adding images to gallery (using proxy - lazy loading):");

    println!("\nDisplaying specific images:");
    gallery.display_image(0);  // This will trigger actual loading
    gallery.display_image(2);  // This will trigger actual loading

    println!("\nDisplaying image 1 again (already loaded):");
    gallery.display_image(0);  // Already loaded, no delay
    println!();
}

// ================================================================================
// BEHAVIORAL PATTERNS
// ================================================================================

// ================================================================================
// 11. OBSERVER PATTERN
// ================================================================================

/**
 * Problem: Define one-to-many dependency between objects so that when one changes state, all dependents are notified.
 * Use Case: Event systems, UI updates, stock price monitoring
 * Rust Approach: Use traits and collections for managing observers
 */

trait Observer {
    fn update(&self, message: &str);
}

trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>);
    fn detach(&mut self, observer_id: usize);
    fn notify(&self, message: &str);
}

struct NewsAgency {
    observers: Vec<(usize, Box<dyn Observer>)>,
    next_id: usize,
    news: String,
}

impl NewsAgency {
    fn new() -> Self {
        NewsAgency {
            observers: Vec::new(),
            next_id: 0,
            news: String::new(),
        }
    }

    fn set_news(&mut self, news: &str) {
        self.news = news.to_string();
        self.notify(&format!("Breaking News: {}", news));
    }
}

impl Subject for NewsAgency {
    fn attach(&mut self, observer: Box<dyn Observer>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.observers.push((id, observer));
        id
    }

    fn detach(&mut self, observer_id: usize) {
        self.observers.retain(|(id, _)| *id != observer_id);
    }

    fn notify(&self, message: &str) {
        for (_, observer) in &self.observers {
            observer.update(message);
        }
    }
}

struct Newspaper {
    name: String,
}

impl Newspaper {
    fn new(name: &str) -> Self {
        Newspaper {
            name: name.to_string(),
        }
    }
}

impl Observer for Newspaper {
    fn update(&self, message: &str) {
        println!("{} received: {}", self.name, message);
    }
}

struct TVChannel {
    channel_name: String,
}

impl TVChannel {
    fn new(name: &str) -> Self {
        TVChannel {
            channel_name: name.to_string(),
        }
    }
}

impl Observer for TVChannel {
    fn update(&self, message: &str) {
        println!("{} breaking news: {}", self.channel_name, message);
    }
}

fn demonstrate_observer() {
    println!("=== OBSERVER PATTERN ===");

    let mut news_agency = NewsAgency::new();

    let ny_times = Newspaper::new("New York Times");
    let guardian = Newspaper::new("The Guardian");
    let cnn = TVChannel::new("CNN");
    let bbc = TVChannel::new("BBC News");

    // Subscribe observers
    let ny_times_id = news_agency.attach(Box::new(ny_times));
    let guardian_id = news_agency.attach(Box::new(guardian));
    let cnn_id = news_agency.attach(Box::new(cnn));

    println!("First news update:");
    news_agency.set_news("Major scientific discovery announced!");

    println!("\nAdding BBC News and second update:");
    let bbc_id = news_agency.attach(Box::new(bbc));
    news_agency.set_news("Stock market reaches record high!");

    println!("\nRemoving CNN and third update:");
    news_agency.detach(cnn_id);
    news_agency.set_news("New technology breakthrough revealed!");
    println!();
}

// ================================================================================
// 12. STRATEGY PATTERN
// ================================================================================

/**
 * Problem: Define a family of algorithms, encapsulate each one, and make them interchangeable.
 * Use Case: Sorting algorithms, payment methods, compression algorithms
 * Rust Approach: Use traits for strategies and structs for context
 */

trait PaymentStrategy {
    fn pay(&self, amount: i32);
}

struct CreditCardPayment {
    name: String,
    card_number: String,
    cvv: String,
    expiry_date: String,
}

impl CreditCardPayment {
    fn new(name: &str, card_number: &str, cvv: &str, expiry_date: &str) -> Self {
        CreditCardPayment {
            name: name.to_string(),
            card_number: card_number.to_string(),
            cvv: cvv.to_string(),
            expiry_date: expiry_date.to_string(),
        }
    }
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: i32) {
        println!("{} paid with credit card.", amount);
        println!("Card holder: {}", self.name);
        let last_four = &self.card_number[self.card_number.len() - 4..];
        println!("Card number: **** **** **** {}", last_four);
    }
}

struct PayPalPayment {
    email: String,
    password: String,
}

impl PayPalPayment {
    fn new(email: &str, password: &str) -> Self {
        PayPalPayment {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: i32) {
        println!("{} paid using PayPal.", amount);
        println!("PayPal account: {}", self.email);
    }
}

struct BitcoinPayment {
    wallet_address: String,
}

impl BitcoinPayment {
    fn new(address: &str) -> Self {
        BitcoinPayment {
            wallet_address: address.to_string(),
        }
    }
}

impl PaymentStrategy for BitcoinPayment {
    fn pay(&self, amount: i32) {
        println!("{} paid using Bitcoin.", amount);
        let short_address = &self.wallet_address[0..10];
        println!("Bitcoin wallet: {}...", short_address);
    }
}

struct ShoppingCart {
    payment_strategy: Option<Box<dyn PaymentStrategy>>,
    total_amount: i32,
}

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart {
            payment_strategy: None,
            total_amount: 0,
        }
    }

    fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.payment_strategy = Some(strategy);
    }

    fn add_item(&mut self, price: i32) {
        self.total_amount += price;
    }

    fn checkout(&self) {
        println!("Total amount: ${}", self.total_amount);
        if let Some(strategy) = &self.payment_strategy {
            strategy.pay(self.total_amount);
        } else {
            println!("Please select a payment method.");
        }
    }
}

fn demonstrate_strategy() {
    println!("=== STRATEGY PATTERN ===");

    let mut cart = ShoppingCart::new();
    cart.add_item(100);
    cart.add_item(50);
    cart.add_item(75);

    println!("Paying with Credit Card:");
    cart.set_payment_strategy(Box::new(CreditCardPayment::new(
        "John Doe", "1234567890123456", "123", "12/25")));
    cart.checkout();

    println!("\nPaying with PayPal:");
    cart.set_payment_strategy(Box::new(PayPalPayment::new(
        "john.doe@example.com", "password123")));
    cart.checkout();

    println!("\nPaying with Bitcoin:");
    cart.set_payment_strategy(Box::new(BitcoinPayment::new(
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")));
    cart.checkout();
    println!();
}

// ================================================================================
// 13. COMMAND PATTERN
// ================================================================================

/**
 * Problem: Encapsulate requests as objects, allowing parameterization and queuing.
 * Use Case: Undo/redo, macro operations, transactional behavior
 * Rust Approach: Use trait objects for commands and structs for invokers
 */

trait Command {
    fn execute(&self);
    fn undo(&self);
}

struct Light {
    is_on: bool,
}

impl Light {
    fn new() -> Self {
        Light { is_on: false }
    }

    fn turn_on(&mut self) {
        self.is_on = true;
        println!("Light is ON");
    }

    fn turn_off(&mut self) {
        self.is_on = false;
        println!("Light is OFF");
    }
}

struct Stereo {
    is_on: bool,
    volume: u32,
}

impl Stereo {
    fn new() -> Self {
        Stereo {
            is_on: false,
            volume: 0,
        }
    }

    fn on(&mut self) {
        self.is_on = true;
        println!("Stereo is ON");
    }

    fn off(&mut self) {
        self.is_on = false;
        println!("Stereo is OFF");
    }

    fn set_cd(&mut self) {
        println!("Stereo is set for CD input");
    }

    fn set_volume(&mut self, volume: u32) {
        self.volume = volume;
        println!("Stereo volume set to {}", volume);
    }
}

struct LightOnCommand {
    light: Arc<Mutex<Light>>,
}

impl LightOnCommand {
    fn new(light: Arc<Mutex<Light>>) -> Self {
        LightOnCommand { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.lock().unwrap().turn_on();
    }

    fn undo(&self) {
        self.light.lock().unwrap().turn_off();
    }
}

struct LightOffCommand {
    light: Arc<Mutex<Light>>,
}

impl LightOffCommand {
    fn new(light: Arc<Mutex<Light>>) -> Self {
        LightOffCommand { light }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        self.light.lock().unwrap().turn_off();
    }

    fn undo(&self) {
        self.light.lock().unwrap().turn_on();
    }
}

struct StereoOnWithCDCommand {
    stereo: Arc<Mutex<Stereo>>,
}

impl StereoOnWithCDCommand {
    fn new(stereo: Arc<Mutex<Stereo>>) -> Self {
        StereoOnWithCDCommand { stereo }
    }
}

impl Command for StereoOnWithCDCommand {
    fn execute(&self) {
        let mut stereo = self.stereo.lock().unwrap();
        stereo.on();
        stereo.set_cd();
        stereo.set_volume(11);
    }

    fn undo(&self) {
        self.stereo.lock().unwrap().off();
    }
}

struct RemoteControl {
    command: Option<Box<dyn Command>>,
    undo_stack: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        RemoteControl {
            command: None,
            undo_stack: Vec::new(),
        }
    }

    fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = Some(command);
    }

    fn button_was_pressed(&mut self) {
        if let Some(command) = self.command.take() {
            command.execute();
            // In a real implementation, you'd need to clone the command
            // This is a simplified version
        }
    }

    fn undo(&mut self) {
        if let Some(command) = self.undo_stack.pop() {
            command.undo();
        }
    }
}

fn demonstrate_command() {
    println!("=== COMMAND PATTERN ===");

    let living_room_light = Arc::new(Mutex::new(Light::new()));
    let stereo = Arc::new(Mutex::new(Stereo::new()));

    let mut remote = RemoteControl::new();

    // Turn on light
    remote.set_command(Box::new(LightOnCommand::new(Arc::clone(&living_room_light))));
    println!("Pressing ON button:");
    remote.button_was_pressed();

    // Turn on stereo
    remote.set_command(Box::new(StereoOnWithCDCommand::new(Arc::clone(&stereo))));
    println!("\nPressing ON button:");
    remote.button_was_pressed();

    // Undo last command
    println!("\nPressing UNDO button:");
    remote.undo();

    println!("\nPressing UNDO button again:");
    remote.undo();
    println!();
}

// ================================================================================
// 14. ITERATOR PATTERN
// ================================================================================

/**
 * Problem: Provide sequential access to elements of an aggregate object without exposing its representation.
 * Use Case: Container classes, collection traversal, data structure access
 * Rust Approach: Use Rust's built-in Iterator trait and IntoIterator
 */

struct NameRepository {
    names: Vec<String>,
}

impl NameRepository {
    fn new() -> Self {
        NameRepository {
            names: vec![
                "Robert".to_string(),
                "John".to_string(),
                "Julie".to_string(),
                "Lora".to_string(),
            ],
        }
    }
}

impl IntoIterator for NameRepository {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

// Custom iterator example
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn demonstrate_iterator() {
    println!("=== ITERATOR PATTERN ===");

    let names_repository = NameRepository::new();

    println!("Names in repository:");
    for name in names_repository {
        println!("Name: {}", name);
    }

    println!("\nCustom iterator:");
    let counter = Counter::new(5);
    for number in counter {
        println!("Number: {}", number);
    }
    println!();
}

// ================================================================================
// 15. TEMPLATE METHOD PATTERN
// ================================================================================

/**
 * Problem: Define the skeleton of an algorithm, deferring some steps to subclasses.
 * Use Case: Frameworks, algorithms with fixed structure, data processing pipelines
 * Rust Approach: Use traits with default implementations and specialized implementations
 */

trait DataProcessor {
    // Template method - defines the algorithm structure
    fn process_data(&mut self) {
        self.load_data();
        if self.validate_data() {
            self.transform_data();
            self.calculate_results();
            self.display_results();
        } else {
            println!("Data validation failed!");
        }
        self.cleanup();
    }

    // Abstract methods - to be implemented by concrete types
    fn load_data(&mut self);
    fn transform_data(&mut self);
    fn calculate_results(&mut self);
    fn display_results(&self);
    fn cleanup(&mut self);

    // Hook method with default implementation
    fn validate_data(&self) -> bool {
        true // Default implementation
    }
}

struct CSVDataProcessor {
    data: Vec<Vec<String>>,
    results: Vec<f64>,
}

impl CSVDataProcessor {
    fn new() -> Self {
        CSVDataProcessor {
            data: Vec::new(),
            results: Vec::new(),
        }
    }
}

impl DataProcessor for CSVDataProcessor {
    fn load_data(&mut self) {
        println!("Loading CSV data...");
        // Simulate loading CSV data
        self.data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
    }

    fn transform_data(&mut self) {
        println!("Transforming CSV data...");
        // Convert strings to numbers and calculate sums
        for row in &self.data {
            let sum: f64 = row.iter()
                .map(|cell| cell.parse::<f64>().unwrap_or(0.0))
                .sum();
            self.results.push(sum);
        }
    }

    fn calculate_results(&mut self) {
        println!("Calculating results...");
        // Calculate average
        let total: f64 = self.results.iter().sum();
        let average = total / self.results.len() as f64;
        self.results.push(average);
    }

    fn display_results(&self) {
        println!("CSV Processing Results:");
        for (i, &result) in self.results.iter().enumerate() {
            if i < self.results.len() - 1 {
                println!("Row {} sum: {:.1}", i + 1, result);
            } else {
                println!("Average: {:.1}", result);
            }
        }
    }

    fn cleanup(&mut self) {
        println!("Cleaning up CSV resources...");
        self.data.clear();
        self.results.clear();
    }
}

struct JSONDataProcessor {
    data: HashMap<String, f64>,
    results: Vec<(String, f64)>,
}

impl JSONDataProcessor {
    fn new() -> Self {
        JSONDataProcessor {
            data: HashMap::new(),
            results: Vec::new(),
        }
    }
}

impl DataProcessor for JSONDataProcessor {
    fn load_data(&mut self) {
        println!("Loading JSON data...");
        // Simulate loading JSON data
        self.data.insert("apple".to_string(), 1.5);
        self.data.insert("banana".to_string(), 0.8);
        self.data.insert("orange".to_string(), 1.2);
    }

    fn validate_data(&self) -> bool {
        println!("Validating JSON data...");
        // Custom validation for JSON
        !self.data.is_empty() && self.data.len() <= 10
    }

    fn transform_data(&mut self) {
        println!("Transforming JSON data...");
        // Apply discount
        for (key, &value) in &self.data {
            self.results.push((key.clone(), value * 0.9)); // 10% discount
        }
    }

    fn calculate_results(&mut self) {
        println!("Calculating results...");
        // Sort by price
        self.results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    fn display_results(&self) {
        println!("JSON Processing Results (after 10% discount):");
        for (item, price) in &self.results {
            println!("{}: ${:.2}", item, price);
        }
    }

    fn cleanup(&mut self) {
        println!("Cleaning up JSON resources...");
        self.data.clear();
        self.results.clear();
    }
}

fn demonstrate_template_method() {
    println!("=== TEMPLATE METHOD PATTERN ===");

    println!("Processing CSV data:");
    let mut csv_processor = CSVDataProcessor::new();
    csv_processor.process_data();

    println!("\nProcessing JSON data:");
    let mut json_processor = JSONDataProcessor::new();
    json_processor.process_data();
    println!();
}

// ================================================================================
// 16. STATE PATTERN
// ================================================================================

/**
 * Problem: Allow an object to change its behavior when its internal state changes.
 * Use Case: State machines, game objects, workflow systems
 * Rust Approach: Use traits for states and enums for state management
 */

trait State {
    fn insert_coin(&mut self, context: &mut GumballMachine);
    fn eject_coin(&mut self, context: &mut GumballMachine);
    fn turn_crank(&mut self, context: &mut GumballMachine);
    fn dispense(&mut self, context: &mut GumballMachine);
    fn get_name(&self) -> &str;
}

struct GumballMachine {
    state: Box<dyn State>,
    count: i32,
}

impl GumballMachine {
    fn new(count: i32) -> Self {
        let mut machine = GumballMachine {
            state: Box::new(NoCoinState::new()),
            count,
        };
        if count == 0 {
            machine.state = Box::new(SoldOutState::new());
        }
        machine
    }

    fn insert_coin(&mut self) {
        self.state.insert_coin(self);
    }

    fn eject_coin(&mut self) {
        self.state.eject_coin(self);
    }

    fn turn_crank(&mut self) {
        self.state.turn_crank(self);
        self.state.dispense(self);
    }

    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }

    fn release_ball(&mut self) {
        println!("A gumball comes rolling out the slot...");
        if self.count != 0 {
            self.count -= 1;
        }
    }

    fn get_count(&self) -> i32 {
        self.count
    }

    fn get_state_name(&self) -> &str {
        self.state.get_name()
    }
}

struct NoCoinState;
impl NoCoinState {
    fn new() -> Self {
        NoCoinState
    }
}

impl State for NoCoinState {
    fn insert_coin(&mut self, context: &mut GumballMachine) {
        println!("You inserted a coin");
        context.set_state(Box::new(HasCoinState::new()));
    }

    fn eject_coin(&mut self, _context: &mut GumballMachine) {
        println!("You haven't inserted a coin");
    }

    fn turn_crank(&mut self, _context: &mut GumballMachine) {
        println!("You turned, but there's no coin");
    }

    fn dispense(&mut self, _context: &mut GumballMachine) {
        println!("You need to pay first");
    }

    fn get_name(&self) -> &str {
        "No Coin"
    }
}

struct HasCoinState;
impl HasCoinState {
    fn new() -> Self {
        HasCoinState
    }
}

impl State for HasCoinState {
    fn insert_coin(&mut self, _context: &mut GumballMachine) {
        println!("You can't insert another coin");
    }

    fn eject_coin(&mut self, context: &mut GumballMachine) {
        println!("Coin returned");
        context.set_state(Box::new(NoCoinState::new()));
    }

    fn turn_crank(&mut self, context: &mut GumballMachine) {
        println!("You turned...");
        context.set_state(Box::new(SoldState::new()));
    }

    fn dispense(&mut self, _context: &mut GumballMachine) {
        println!("No gumball dispensed");
    }

    fn get_name(&self) -> &str {
        "Has Coin"
    }
}

struct SoldState;
impl SoldState {
    fn new() -> Self {
        SoldState
    }
}

impl State for SoldState {
    fn insert_coin(&mut self, _context: &mut GumballMachine) {
        println!("Please wait, we're already giving you a gumball");
    }

    fn eject_coin(&mut self, _context: &mut GumballMachine) {
        println!("Sorry, you already turned the crank");
    }

    fn turn_crank(&mut self, _context: &mut GumballMachine) {
        println!("Turning twice doesn't get you another gumball!");
    }

    fn dispense(&mut self, context: &mut GumballMachine) {
        context.release_ball();
        if context.get_count() > 0 {
            context.set_state(Box::new(NoCoinState::new()));
        } else {
            println!("Oops, out of gumballs!");
            context.set_state(Box::new(SoldOutState::new()));
        }
    }

    fn get_name(&self) -> &str {
        "Sold"
    }
}

struct SoldOutState;
impl SoldOutState {
    fn new() -> Self {
        SoldOutState
    }
}

impl State for SoldOutState {
    fn insert_coin(&mut self, _context: &mut GumballMachine) {
        println!("You can't insert a coin, the machine is sold out");
    }

    fn eject_coin(&mut self, _context: &mut GumballMachine) {
        println!("You can't eject, you haven't inserted a coin yet");
    }

    fn turn_crank(&mut self, _context: &mut GumballMachine) {
        println!("You turned, but there are no gumballs");
    }

    fn dispense(&mut self, _context: &mut GumballMachine) {
        println!("No gumball dispensed");
    }

    fn get_name(&self) -> &str {
        "Sold Out"
    }
}

fn demonstrate_state() {
    println!("=== STATE PATTERN ===");

    let mut gumball_machine = GumballMachine::new(5);

    println!("{}", gumball_machine.get_state_name());

    gumball_machine.insert_coin();
    println!("{}", gumball_machine.get_state_name());

    gumball_machine.turn_crank();
    println!("{}", gumball_machine.get_state_name());
    println!("Inventory: {}", gumball_machine.get_count());

    gumball_machine.insert_coin();
    gumball_machine.turn_crank();
    println!("Inventory: {}", gumball_machine.get_count());

    gumball_machine.insert_coin();
    gumball_machine.eject_coin();
    println!("{}", gumball_machine.get_state_name());
    println!();
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE RUST DESIGN PATTERNS               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    println!("CREATIONAL PATTERNS");
    println!("====================");
    demonstrate_singleton();
    demonstrate_factory_method();
    demonstrate_abstract_factory();
    demonstrate_builder();
    demonstrate_prototype();

    println!("STRUCTURAL PATTERNS");
    println!("====================");
    demonstrate_adapter();
    demonstrate_decorator();
    demonstrate_facade();
    demonstrate_flyweight();
    demonstrate_proxy();

    println!("BEHAVIORAL PATTERNS");
    println!("====================");
    demonstrate_observer();
    demonstrate_strategy();
    demonstrate_command();
    demonstrate_iterator();
    demonstrate_template_method();
    demonstrate_state();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This implementation covers essential design patterns in Rust:");
    println!();
    println!("📁 CREATIONAL PATTERNS:");
    println!("   • Singleton: Thread-safe single instance using OnceLock");
    println!("   • Factory Method: Trait-based object creation");
    println!("   • Abstract Factory: Families of related objects");
    println!("   • Builder: Method chaining for complex construction");
    println!("   • Prototype: Clone trait for object copying");
    println!();
    println!("🏗️  STRUCTURAL PATTERNS:");
    println!("   • Adapter: Trait-based interface adaptation");
    println!("   • Decorator: Wrapper structs with composition");
    println!("   • Facade: Simplified interface to complex systems");
    println!("   • Flyweight: Arc for shared data optimization");
    println!("   • Proxy: Interior mutability for lazy loading");
    println!();
    println!("🎯 BEHAVIORAL PATTERNS:");
    println!("   • Observer: Trait-based event notification");
    println!("   • Strategy: Interchangeable algorithms");
    println!("   • Command: Encapsulated requests with Arc<Mutex<>>");
    println!("   • Iterator: Built-in Iterator trait");
    println!("   • Template Method: Trait with default implementations");
    println!("   • State: Dynamic behavior change with trait objects");
    println!();
    println!("Rust-specific considerations:");
    println!("✅ Memory safety through ownership system");
    println!("✅ Thread safety with Arc, Mutex, RwLock");
    println!("✅ Zero-cost abstractions");
    println!("✅ Pattern matching for expressive control flow");
    println!("✅ Trait system for polymorphism");
    println!("✅ No null pointers - Option<T> instead");
    println!("✅ Error handling with Result<T, E>");
}

/*
================================================================================
COMPREHENSIVE RUST DESIGN PATTERNS SUMMARY:
================================================================================

CREATIONAL PATTERNS:
• Singleton: Thread-safe using OnceLock, global access point
• Factory Method: Trait-based polymorphic creation
• Abstract Factory: Families of related objects with traits
• Builder: Method chaining for fluent construction
• Prototype: Clone trait for efficient copying

STRUCTURAL PATTERNS:
• Adapter: Trait compatibility and wrapper structs
• Decorator: Composition with newtypes and wrappers
• Facade: Simplified interface to complex subsystems
• Flyweight: Arc for memory-efficient sharing
• Proxy: Interior mutability with RefCell/Mutex

BEHAVIORAL PATTERNS:
• Observer: Trait-based subscription system
• Strategy: Interchangeable algorithms via traits
• Command: Encapsulated operations with smart pointers
• Iterator: Built-in Iterator trait and IntoIterator
• Template Method: Traits with default implementations
• State: Dynamic behavior with trait objects

RUST-SPECIFIC FEATURES:
• Ownership system for memory safety
• Pattern matching for expressive control flow
• Trait system for compile-time polymorphism
• Smart pointers for thread-safe sharing
• Option and Result for null/error safety
• Zero-cost abstractions
• Powerful macro system

COMPILATION:
rustc 03-Rust_Design_Patterns.rs -o patterns_demo

USAGE:
./patterns_demo

Each pattern demonstrates:
✅ Problem definition and Rust-specific solution
✅ Idiomatic Rust implementations
✅ Thread safety where applicable
✅ Memory safety and efficiency
✅ Practical real-world examples
✅ Clear separation of concerns
================================================================================
*/