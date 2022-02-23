use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::env;
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]

pub struct VoteContract {
    // SETUP CONTRACT STATE
    candidates: UnorderedMap<String,i128>,
    voters: UnorderedMap<String,String>
}

impl Default for VoteContract {
    fn default() -> Self {
        Self {
            candidates: UnorderedMap::new(b"a"),
            voters: UnorderedMap::new(b"b")
        }
    }
}

#[near_bindgen]
impl VoteContract {
    // ADD CONTRACT METHODS HERE
    pub fn add_candidate(&mut self, name: String) -> String{
        
        if !self.candidates.get(&name).is_none(){
            return "Candidate already in the list.".to_string();
        }

        self.candidates.insert(&name,&0);
        
        return ["Candidate",&name,"Added."].join(" ");
    }

    pub fn get_candidates(&self) -> Vec<String>{
        return self.candidates.keys_as_vector().to_vec().clone();
    }

    pub fn show_results(&self) -> Vec<(String,i128)>{

        if self.voters.get(&env::signer_account_id().to_string()).is_none() {
            return [("You must vote first to show the results.".to_string(),0)].to_vec();
        }

        return self.candidates.iter().map(|x|x).collect();
    }

    pub fn show_voters(&self) -> Vec<(String,String)>{

        if self.voters.get(&env::signer_account_id().to_string()).is_none() {
            return [("You must vote first to show the voters.".to_string(),"".to_string())].to_vec();
        }

        return self.voters.iter().map(|x|x).collect();
    }

    pub fn add_vote(&mut self, name: String) -> String{

        if self.candidates.get(&name).is_none(){
            return "Candidate not found!".to_string();
        }

        let mut thing = String::new();
        
        thing.push_str(&env::signer_account_id().to_string());
        thing.push_str(&name);

        if !self.voters.get(&thing).is_none() {
            return "You already voted for this candidate before!".to_string();
        }

        let mut val = self.candidates.get(&name).unwrap();
        val += 1;

        self.candidates.insert(&name,&val);

        self.voters.insert(&thing.to_string(), &name);
            
        return ["You voted for",&name,"successfully.","Total votes:", &val.to_string()].join(" ");
    }
}

