pub fn custom_filter<T, F>(array: Vec<T>, mut call_back: F ) ->Vec<T>
where
    F :  FnMut(&T) -> bool,
        {
        let mut result:Vec<T> = Vec::new();

        for item in array.into_iter(){
            if call_back(&item){
                result.push(item);
            }
          
        }
        result
}


