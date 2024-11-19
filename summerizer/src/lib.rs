/*
This is a library crate that will contain the main functionality of the summerizer crate.
It will use the rust-bert library to perform text summarization.
*/
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::RustBertError;

// Define a struct to hold the summarization model
pub struct Summerizer {
    model: SummarizationModel,
}

impl Summerizer {
    // Create a new instance of the Summerizer struct
    pub fn new() -> Result<Self, RustBertError> {
        // Load the summarization model
        let model = SummarizationModel::new(Default::default())?;

        Ok(Self { model })
    }

    // Summarize the input text
    pub fn summarize(&self, text: &str) -> Result<String, RustBertError> {
        let summaries = self.model.summarize(&[text])?;
        Ok(summaries[0].clone())
    }
}

// Define a function to create a new instance of the Summerizer struct
pub fn new_summerizer() -> Result<Summerizer, RustBertError> {
    Summerizer::new()
}

// Define a function to summarize the input text using the provided Summerizer instance
pub fn summarize_text(summerizer: &Summerizer, text: &str) -> Result<String, RustBertError> {
    summerizer.summarize(text)
}
