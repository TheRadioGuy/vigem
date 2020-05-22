#[cfg(test)]
mod tests {
    use vigem::*;
    #[test]
    fn make_xbox_controller() {
        let mut vigem = Vigem::new();
        vigem.connect().unwrap();
        let target = Target::new(TargetType::Xbox360);
        assert!(matches!(target.state(), TargetState::Initialized));
        vigem.target_add(&target).unwrap();
        assert!(matches!(target.state(), TargetState::Connected));
    }
}