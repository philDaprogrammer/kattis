#include <iostream>
#include <sstream>
#include <cstdio>
#include <cmath>
using namespace std; 

const int MAX_INT = 1 << 30; 

/**
 * Did this one in CPP, as it still 
 * craps on rust in terms of speed, 
 * and we need to be as fast as possible 
 * for this problem
 *
 * Solution to 1s for all on kattis. 
 * The given solution runs in O(n^2) 
 * time and manages to not time out. 
 * We have 15 seconds of CPU time,  
 * and this passed with 5 seconds to 
 * spare. I am not sure if it is 
 * possible to get a theoretically 
 * faster algorithm, but this will 
 * suffice the problem constraints. 
 *
 */ 
int main(void) { 
   int n; 
   cin >> n; 
   int opts[n+1] = {0};
    
   stringstream split; 

   if (n <= 5) {
      cout << n << endl; 
      return 0; 
   }
   
   // set the base cases for our dynamic program  
   opts[0] = 0; 
   opts[1] = 1; 
   opts[2] = 2; 
   opts[3] = 3; 
   opts[4] = 4;  
   opts[5] = 5; 

   int contender; 
   for (int i=6; i <=n ; ++i) { 
      opts[i] = MAX_INT; 
      
      // get the optimal ones complexity for i 
      for (int j=2; j < i ; ++j) { 

         contender = j <= (i / 2) 
            ? opts[i / j] + opts[j] + opts[i % j] // case for multiplying
            : opts[j] + opts[i - j];              // case for adding 

         if (contender < opts[i]) {
            opts[i] = contender;
         }  
      }

      string digits = to_string(i);

      // can we do better with concat?
      for (int j=1; j < digits.length(); j++) { 
         int left, right; 
         split.clear(); 

         split << digits.substr(0, j) << ' ' << digits.substr(j);
         split >> left >> right; 

         if (opts[left] + opts[right] < opts[i] && digits[j] != '0') { 
            opts[i] = opts[left] + opts[right]; 
         } 
      } 
   }  

   cout << opts[n] << endl; 
   return  0; 
} 
