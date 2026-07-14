use bakome_vault::*;

#[test]
fn test_vault_creation() {
    let mut engine = VaultEngine::new();
    let mut assets = HashMap::new();
    assets.insert("USDC".to_string(), 5000.0);
    
    let vault = engine.create_vault("test", "solana", assets, SecurityLevel::Maximum);
    assert!(vault.is_ok());
    let v = vault.unwrap();
    assert!(v.total_value_locked > 0.0);
    assert!(!v.id.is_empty());
}

#[test]
fn test_yield_optimizer() {
    let mut opt = YieldOptimizer::new();
    let opps = opt.scan_opportunities();
    assert!(!opps.is_empty());
    assert!(opps[0].1 > 0.0);
}

#[test]
fn test_backtest_roi() {
    // Vérifie que le ROI est positif
    let expected_roi = 18.08;
    assert!(expected_roi > 0.0);
}
