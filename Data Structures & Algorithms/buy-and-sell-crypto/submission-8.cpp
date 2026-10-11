class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int max_profit = 0;

        int l = 0;
        int r = 1;
        if(prices.size() == 1) return 0;

        while (r < prices.size())
        {
            if(prices[l] > prices[r])
            {
                l = r;
                r += 1;
            }
            else
            {
                int p = prices[r] - prices[l];
                max_profit = max(p,max_profit);
              
                r++;
            }
        }
        return max_profit;
        
    }
};
/*

 
*/