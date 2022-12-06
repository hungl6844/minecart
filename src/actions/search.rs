use crate::data;


pub async fn exec(args: Vec<String>) {
    let result = data::search(args.join(" ").as_str()).await;

    for provider in result {
        for resource in provider {
            println!("{}", resource["name"]);
        }


    }


    
}


