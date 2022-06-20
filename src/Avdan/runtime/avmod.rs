use std::{collections::HashMap, hash::Hash, path::{Path, PathBuf}, fs, thread, time::Duration, };

use colored::Colorize;
use v8::{Module, Global, ModuleStatus, TryCatch, HandleScope, ScriptOrigin, script_compiler::Source, Local, Context, FixedArray, CallbackScope, ModuleRequest, Exception, };

use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name : String,
    version: String,

    #[serde(default = "Package::default_main")]
    main: String,

    #[serde(default = "Package::default_browser")]
    browser: bool,
    dependencies: HashMap<String, String>
}

impl Package {
    pub fn default_main() -> String {
        "./index.js".into()
    }

    pub fn default_browser() -> bool {
        false
    } 
}

#[derive(PartialEq, Eq)]
pub enum ResourceIdentifier {
    FilePath(String), // 
    Internal(String), // 
    Package(String),  // 
}

impl ResourceIdentifier {
    pub fn from(identifier: String) -> ResourceIdentifier {
        match identifier {
            id if id.starts_with(".") || id.starts_with("/") => Self::FilePath(id),
            id if id.starts_with("@avdan") => Self::Internal(id.replace("@avdan", "")),
            id => Self::Package(id),
        }
    }

    pub fn resolve<'a>(&self, scope : &mut HandleScope<'a>,  current_dir : String) -> Result<PathBuf, String> {
        match self {
            Self::FilePath(path) => Self::resolve_file(scope, current_dir, path.clone()),
            Self::Internal(name) => todo!(),
            Self::Package(name) => Self::resolve_package(scope, current_dir, name.clone())
        }
    }

    fn resolve_file<'a>(scope : &mut HandleScope<'a>, current_dir: String, file_name: String) -> Result<PathBuf, String> {
        // TODO: Remove this
        let path = Path::new(current_dir.as_str());
        let path = path.join(file_name);
        println!("Resolving file {:?}", path.clone());

        match path.exists() {
            true => match path {
                p if p.is_dir() => Err(format!("{} is a directory!", p.to_str().unwrap())),
                p => Ok(p)
            },
            false => Err("File does not exist!".to_string())
        }
    }

    // Load the package through node_modules
    fn resolve_package_folder<'a>(current_dir : String, id: String) -> Result<PathBuf, String> {
        let mut path = Path::new(current_dir.as_str());
        
        loop {
            let dir = fs::read_dir(path);
            match dir {
                Err(e) => panic!("Directory doesn't exist! {:?}", path),
                Ok(mut dir) => {
                    let contains_node_modules = dir.any(|f| {
                        let file = f.unwrap();
                        file.file_type().unwrap().is_dir() && file.file_name().eq("node_modules".into())
                    });

                    if contains_node_modules {
                        let node_modules_path = path.join("node_modules");
                        let mut node_modules = fs::read_dir(node_modules_path.clone()).unwrap();
                        
                        let module_dir = node_modules.find(|f| {
                            match f {
                                Err(_) => false,
                                Ok(file) => file.file_name().to_string_lossy().eq(&id) 
                            }
                        });

                        return match module_dir {
                            None => Err(format!("Couldn't find package {} in `node_modules` !", id)),
                            Some(dir) => match dir {
                                Err(err) => Err(err.to_string()),
                                Ok(dir) => {
                                    let tmp = node_modules_path.join(dir.path());
                                    Ok(tmp)
                                }
                            }
                        }
                    }

                    let tmp_path = path.parent();
                    match tmp_path {
                        None => return Err("Couldn't find `node_modules` folder!".into()),
                        Some(p) => {
                            path = p;
                        } 
                    };
                }
            }
        };
    }

    fn parse_package_json<'a>(module_root : &'a Path) -> Result<Package, String>{
        let package_json_file =  fs::read_to_string(module_root.join("package.json"));

        if package_json_file.is_err() {
            return Err(format!("Cannot find `package.json` file in {}", module_root.to_string_lossy()));
        }

        let package_json_file = package_json_file.unwrap();

        let _package_json = serde_json::from_str(&package_json_file);

        match _package_json {
            Err(err) => Err(err.to_string()),
            Ok(package_json) => Ok(package_json)
        }
    }

    fn resolve_package<'a>(scope: &mut HandleScope<'a>, current_dir : String, id : String) -> Result<PathBuf, String> {
        let module_root = Self::resolve_package_folder(current_dir, id)?;
        let package_json = Self::parse_package_json(&module_root)?;
        Ok(module_root.join(package_json.main))
    }

    fn string_id(&self) -> String {
        match self {
            Self::FilePath(path) => path.clone(),
            Self::Internal(name) => name.clone(),
            Self::Package(name) => name.clone()
        }
    }
}

impl ToString for ResourceIdentifier {
    fn to_string(&self) -> String {
        Self::string_id(self)
    }
}

impl Into<String> for ResourceIdentifier {
    fn into(self) -> String {
        Self::string_id(&self)
    }
}

impl Clone for ResourceIdentifier {
    fn clone(&self) -> Self {
        match self {
            Self::FilePath(arg0) => Self::FilePath(arg0.clone()),
            Self::Internal(arg0) => Self::Internal(arg0.clone()),
            Self::Package(arg0) => Self::Package(arg0.clone()),
        }
    }
}

impl Hash for ResourceIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::FilePath(path) => path.hash(state),
            Self::Internal(identifier) => identifier.hash(state),
            Self::Package(identifier) => identifier.hash(state)
        }
        self.hash(state)
    }
}

type Identifier = ResourceIdentifier;


// Stores all modules, and all the information about them.
pub struct ModuleStore {
    status  : HashMap<String, Global<Module>>,
}

impl ModuleStore {
    pub fn new() -> ModuleStore {
        ModuleStore { status: HashMap::new() }
    }

    pub fn contains(&self, id: &Identifier) -> bool {
        self.status.contains_key(&id.string_id())
    }


    pub fn add(&mut self, module: &AvModule,) -> () {
        println!("#1");
        match self.contains(&module.identifier) {
            true => return,
            false => {
                println!("#2");
                let c = module.identifier.string_id();
                println!("#3");
                self.status.insert(c, module.module.as_ref().unwrap().clone());
                println!("#4");
            }
        }
    }

}

pub struct AvModule {
    identifier : Identifier,
    dependencies: Vec<Identifier>,
    module: Option<Global<Module>>
}

impl AvModule {

    pub fn new(id : Identifier) -> AvModule {
        AvModule { identifier: id, dependencies: vec![], module: None }
    }

    pub fn add_dependency(&mut self, module : Identifier) -> () {
        self.dependencies.push(module);
    }

    pub fn default_origin<'s>(scope: &mut HandleScope<'s>, name: String) -> ScriptOrigin<'s> {
        let name = v8::String::new(scope, name.as_str()).unwrap();
        let udef = v8::undefined(scope);
        ScriptOrigin::new(scope,
            name.into(),
            0,
            0,
            false,
            1,
            udef.into(),
            false,
            false,
            true
        )
    }

    pub fn independent(&self) -> bool {
        self.dependencies.is_empty() 
    }

    pub fn resolve(&mut self, scope : &mut HandleScope, current_dir: String) -> Result<Global<Module>, String> {
        println!("{}", Colorize::blue("AvModule::resolve\n"));
        let path_to_module = self.identifier.resolve(scope, current_dir.clone())?;
        println!("Resolving module with path: {:?}", path_to_module);
        let origin = Self::default_origin(scope, self.identifier.clone().into());

        let source_code = fs::read_to_string(path_to_module.clone());
        if source_code.is_err() {
            return Err(source_code.err().unwrap().to_string());
        } 

        let source_code = v8::String::new(scope, source_code.unwrap().as_str()).unwrap();

        let code = Source::new(source_code, Some(&origin));

        let try_catch = &mut TryCatch::new(scope);

        let module = v8::script_compiler::compile_module(try_catch, code);

        if try_catch.has_caught() {
            let excep = try_catch.exception().unwrap();
            return Err(excep.to_rust_string_lossy(try_catch));
        }

        let module = module.unwrap();

        let global_module = Global::new(try_catch, module);

        let store = try_catch.get_slot_mut::<ModuleStore>().unwrap();

        self.module = Some(global_module.clone());
        store.add(&*self);
       

        println!("Added to store! {}", store.status.len());


        let module_requests = module.get_module_requests();
        let reqs : Vec<ModuleRequest> = vec![];


        // Loop through the `import` statements.
        for i in 0..module_requests.length() {
            let req : Local<ModuleRequest> = module_requests.get(try_catch, i).unwrap().try_into().unwrap();

            let import_specifier = req.get_specifier().to_rust_string_lossy(try_catch);

            println!("Importing : {}\n", import_specifier);
            
            let import_specifier = ResourceIdentifier::from(import_specifier);
            let mut dep_module = AvModule::new(
                import_specifier.clone()
            );

            let m = dep_module.resolve(try_catch, path_to_module.clone().parent().unwrap().to_str().unwrap().into()).unwrap();

            self.add_dependency(import_specifier);
        
        }

        println!("Dependencies : {:?}", self.dependencies.len());

        
        // Todo: Wait for module requests.

        match module.instantiate_module(try_catch, AvModule::callback) {
            None => {
                let excep = try_catch.exception().unwrap();
                panic!("\n\t{}\n", Colorize::red(excep.to_rust_string_lossy(try_catch).as_str()));
            },
            Some(_) => ()
        };

        


        


        Ok(global_module)
    }

    pub fn callback<'b>(context : Local<'b, Context>, specifier : Local<'b, v8::String>, import_assertions : Local<'b, FixedArray>, referrer : Local<'b, Module>) -> Option<Local<'b, Module>> {
        let scope = &mut unsafe {
            CallbackScope::new(context)
        };

        println!("Module: {}.\nHas been instantiated!\n", specifier.to_rust_string_lossy(scope));

        Some(referrer)
    }
}