use sim_model_rs::domain::Searcher;
use sim_model_rs::model::SearchCondition;
use sim_model_rs::service::Simulator;

fn main() {
    println!("MHWilds Equip Simulator (Rust) - Build Check");

    // テスト用の条件を生成
    let condition = SearchCondition::new();
    
    // シミュレータのインスタンス生成
    let mut simulator = Simulator::new();
    
    // (現在はCSVをロードしていないためエラーになるか0件になります)
    // 動作確認のためだけなので、とりあえず呼び出すだけ
    println!("Starting search...");
    let results = simulator.search(&condition, 10);
    
    println!("Search complete. Found {} results.", results.len());
}
