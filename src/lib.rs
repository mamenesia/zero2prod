pub mod configuration;
pub mod routes;
pub mod startup;

// Export the run function from startup module
pub use startup::run;
