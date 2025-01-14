use fn_error_context::context;
use formality_prove::Env;
use formality_rust::grammar::{
    AssociatedTy, AssociatedTyBoundData, Fn, Trait, TraitBoundData, TraitItem, WhereClause,
};
use formality_types::grammar::Fallible;

impl super::Check<'_> {
    #[context("check_trait({:?})", t.id)]
    pub(super) fn check_trait(&self, t: &Trait) -> Fallible<()> {
        let Trait { id: _, binder } = t;
        let mut env = Env::default();

        let TraitBoundData {
            where_clauses,
            trait_items,
        } = env.instantiate_universally(&binder.explicit_binder);

        self.check_trait_items_have_unique_names(&trait_items)?;

        self.prove_where_clauses_well_formed(&env, &where_clauses, &where_clauses)?;

        for trait_item in &trait_items {
            self.check_trait_item(&env, &where_clauses, trait_item)?;
        }

        Ok(())
    }

    fn check_trait_items_have_unique_names(&self, _trait_items: &[TraitItem]) -> Fallible<()> {
        // FIXME:
        Ok(())
    }

    fn check_trait_item(
        &self,
        env: &Env,
        where_clauses: &[WhereClause],
        trait_item: &TraitItem,
    ) -> Fallible<()> {
        match trait_item {
            TraitItem::Fn(v) => self.check_fn_in_trait(env, where_clauses, v),
            TraitItem::AssociatedTy(v) => self.check_associated_ty(env, where_clauses, v),
        }
    }

    fn check_fn_in_trait(&self, env: &Env, where_clauses: &[WhereClause], f: &Fn) -> Fallible<()> {
        self.check_fn(env, where_clauses, f)
    }

    fn check_associated_ty(
        &self,
        trait_env: &Env,
        trait_where_clauses: &[WhereClause],
        associated_ty: &AssociatedTy,
    ) -> Fallible<()> {
        let mut env = trait_env.clone();

        let AssociatedTy { id: _, binder } = associated_ty;
        let AssociatedTyBoundData {
            ensures: _,
            where_clauses,
        } = env.instantiate_universally(binder);

        self.prove_where_clauses_well_formed(
            &env,
            (trait_where_clauses, &where_clauses),
            &where_clauses,
        )?;

        // FIXME: Do we prove ensures WF? And what do we assume when we do so?

        Ok(())
    }
}
