impl Solution {
    pub fn backtrack(r: &mut Vec<Vec<i32>>, i: usize, arr: &Vec<i32>,subset: &mut Vec<i32>){
        if i>=arr.len(){
            r.push(subset.clone());
            return;
        }
        subset.push(arr[i]);
        Self::backtrack(r,i+1,arr,subset);
        subset.pop();
        Self::backtrack( r,i+1,arr,subset);

    }  
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut subset = Vec::new();

        Self::backtrack(&mut results,0,&nums,&mut subset);

        results
    }

     
}

/*


backtracking - i,arr,results



*/