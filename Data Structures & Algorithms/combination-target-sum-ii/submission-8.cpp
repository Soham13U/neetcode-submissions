class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        sort(candidates.begin(),candidates.end());
        vector<vector<int>> ans;
        vector<int> temp;

        backtrack(ans,temp,candidates,target,0,0);

        return ans;
    }

    void backtrack(vector<vector<int>>& ans, vector<int>& temp, vector<int>& candidates, int target, int sum, int i)
    {
         if( sum == target)
        {
            ans.push_back(temp);
            return;
        }
         if( i >= candidates.size() || sum > target)
        {
            return;
        }
        
        sum += candidates[i];
        temp.push_back(candidates[i]);
        backtrack(ans,temp,candidates,target,sum,i+1);
        
        sum -= candidates[i];
        temp.pop_back();
        while(  i + 1 < candidates.size() && candidates[i] == candidates[i+1] )
        {
            i++;
        }
        backtrack(ans,temp,candidates,target,sum,i+1);

    }
};
