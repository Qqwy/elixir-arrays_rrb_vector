use rustler::env::OwnedEnv;
use rustler::env::SavedTerm;
use rustler::Env;
use rustler::Term;

/// A Term which is stored in a separate `OwnedEnv`, Reference-Counted.
/// While creating a TermBox is a little expensive since an `OwnedEnv` needs to be constructed,
/// it is useful since it allows storing _any_ Erlang term inside
/// (including those otherwise unhandleable, such as References, Ports, Functions and Bignums)
#[derive(Clone)]
pub struct TermBox {
    inner: std::sync::Arc<TermBoxContents>,
}

struct TermBoxContents {
    owned_env: OwnedEnv,
    saved_term: SavedTerm,
}
// I believe this is OK since we never alter the TermBox
// once it is created.
// c.f. http://erlang.org/doc/man/erl_nif.html, 'Threads and concurrency'
unsafe impl Sync for TermBoxContents {}

impl TermBox {
    pub fn new(term: &Term) -> Self {
        Self {
            inner: std::sync::Arc::new(TermBoxContents::new(term)),
        }
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
        Self {
            owned_env: owned_env,
            saved_term: saved_term,
        }
    }
}
