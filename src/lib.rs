pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
// Export the run function from startup module
pub use startup::run;
