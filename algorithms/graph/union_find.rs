//DSU disjoint set union or union find is approach used to detect cycle when 
// we have to connect components of graph or circular detection
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0,0];
       let n = edges.iter().flatten().copied().max().unwrap() as usize;
       let mut parent = vec![0;n+1];
       for i in 0..n + 1{
        parent[i] = i;
       }
        fn find(parent: &mut Vec<usize> , x: usize) -> usize {
            if parent[x] != x{
             
              parent[x] = find(parent , parent[x]);
            }
            parent[x]
        }
        
        //if already connected may be undirectly connected it gives true means cycle detected
       fn union(parent: &mut Vec<usize> , u: usize , v: usize ) -> bool{
               let pu= find(parent , u);
               let pv = find(parent , v);

               if pu == pv{
                return true;
               } 

               parent[pv] = pu;
               return false
       }


        for edge in edges{
            let u = edge[0] as usize;
            let v = edge[1] as usize;

           if union(&mut parent , u , v){
            return edge
           }
        }

        vec![]
    }
}