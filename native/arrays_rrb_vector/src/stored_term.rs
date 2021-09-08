    use rustler::env::OwnedEnv;
    use rustler::env::SavedTerm;
    use rustler::Encoder;
    use rustler::Env;
    use rustler::Term;
    use rustler::LocalPid;
    use rustler::types::tuple::get_tuple;
    use rustler::types::tuple::make_tuple;
    use rustler::types::atom::Atom;

// Put this in a ResourceArc:
#[derive(Clone)]
pub struct TermBox
{
    inner: std::sync::Arc<TermBoxContents>,
}

struct TermBoxContents
{
    owned_env: OwnedEnv,
    saved_term: SavedTerm
}
// I believe this is OK since we never alter the TermBox
// once it is created.
unsafe impl Sync for TermBoxContents {}

impl TermBox {
    pub fn new(term: &Term) -> Self {
        Self{inner: std::sync::Arc::new(TermBoxContents::new(term))}
    }

    pub fn get<'a>(&self, env: Env<'a>) -> Term<'a> {

        // Copy over term from owned environment to the target environment
        self.inner.owned_env.run(|inner_env| {
            let term = self.inner.saved_term.load(inner_env);
            term.in_env(env)
        })
    }
}

impl TermBoxContents {
    fn new(term: &Term) -> Self {
        let owned_env = OwnedEnv::new();
        let saved_term = owned_env.save(*term);
        Self{owned_env: owned_env, saved_term: saved_term}
    }
}



#[derive(Clone)]
pub enum StoredTerm {
    Integer(i64),
    Float(f64),
    AnAtom(Atom),
    Tuple(Vec<StoredTerm>),
    EmptyList(),
    List(Vec<StoredTerm>),
    Bitstring(String),
    Pid(LocalPid),
    Other(TermBox),
}

impl Encoder for StoredTerm {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            StoredTerm::Integer(inner) => inner.encode(env),
            StoredTerm::Float(inner) => inner.encode(env),
            StoredTerm::AnAtom(inner) => inner.encode(env),
            StoredTerm::Tuple(inner) => {
                let terms: Vec<_> = inner.iter().map(|t| t.encode(env)).collect();
                make_tuple(env, terms.as_ref()).encode(env)
            }
            StoredTerm::EmptyList() => rustler::Term::list_new_empty(env),
            StoredTerm::List(inner) => inner.encode(env),
            StoredTerm::Bitstring(inner) => inner.encode(env),
            StoredTerm::Pid(inner) => inner.encode(env),
            StoredTerm::Other(inner) => inner.get(env),
        }
    }
}


pub fn convert_to_supported_term(term: &Term) -> StoredTerm {
    match term.get_type() {
        rustler::TermType::Atom => term.decode().map(StoredTerm::AnAtom).unwrap(),
        rustler::TermType::Binary => term.decode().map(StoredTerm::Bitstring).unwrap(),
        rustler::TermType::EmptyList => StoredTerm::EmptyList(),
        rustler::TermType::List => {
            let items = term.decode::<Vec<Term>>().unwrap();
            let converted_items =
                items
                .iter()
                .map(convert_to_supported_term)
                .collect();

            StoredTerm::List(converted_items)
        },
        rustler::TermType::Number =>
            term.decode::<i64>().map(StoredTerm::Integer)
            .or(term.decode::<f64>().map(StoredTerm::Float))
            .unwrap_or(StoredTerm::Other(TermBox::new(term))),
        rustler::TermType::Tuple => {
            let elems = get_tuple(*term).unwrap();
            let converted_elems =
                elems
                .iter()
                .map(convert_to_supported_term)
                .collect();
            StoredTerm::Tuple(converted_elems)
        }
        rustler::TermType::Pid => term.decode().map(StoredTerm::Pid).unwrap(),
        _other => StoredTerm::Other(TermBox::new(term)),
    }
}