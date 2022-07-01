mod api;
mod classifier;

#[cfg(test)]
mod tests {
    use crate::classifier::Classifier;

    #[test]
    fn regression() {
        let mut model = Classifier::new();

        // Simulate populating data by manually doing it here
        let data: Vec<i16> = vec![2, 8, 6, 4, 9, 3, 12];
        model.data = data;

        let labels = vec![0, 3, 3, 2, 4, 1, 9];
        model.labels = labels;

        model.train();

        let prediction = model.predict(20);

        if let Some(prediction) = prediction {
            assert_eq!(prediction, 14);
        } else {
            panic!();
        }
    }
}
