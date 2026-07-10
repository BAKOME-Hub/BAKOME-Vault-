mod vault;
use vault::*;

fn main() {
    println!("🔒 {} — Multi-Chain Non-Custodial Vault", VERSION);
    let mut engine = VaultEngine::new();
    
    // Créer un vault de test
    let mut assets = HashMap::new();
    assets.insert("USDC".to_string(), 10000.0);
    assets.insert("SOL".to_string(), 50.0);
    
    match engine.create_vault("Bakome", "solana", assets, SecurityLevel::Maximum) {
        Ok(vault) => {
            println!("✅ Vault créé: {}", vault.id);
            println!("💰 TVL: ${:.2}", vault.total_value_locked);
            
            // Audit de sécurité
            let audit = SecurityAuditor::audit_vault(&vault);
            println!("🛡️ Score de sécurité: {:.1}%", audit.score);
            
            // Scan de yield
            let opportunities = engine.yield_optimizer.scan_opportunities();
            println!("📊 {} opportunités de yield trouvées", opportunities.len());
            
            // Top 3
            for (i, opp) in opportunities.iter().take(3).enumerate() {
                println!("  {}. {} sur {} — APY: {:.1}%", i+1, opp.protocol, opp.chain, opp.apy);
            }
        }
        Err(e) => eprintln!("❌ Erreur: {}", e),
    }
}
