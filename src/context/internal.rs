use super::*;


/// Internal interface functions
impl Context {
    pub (crate) fn insert_into_root_index(&self, subject_id: SubjectId, subject: &Subject) {
        if let Some(ref index) = *self.root_index.write().unwrap() {
            // TODO3: Trait IndexKey or Hashable
            index.insert(self, subject_id.id, subject);
        } else {
            panic!("no root index")
        }
    }

    /// Called by the Slab whenever memos matching one of our subscriptions comes in, or by the Subject when an edit is made
    pub (crate) fn apply_head(&self, head: &MemoRefHead) -> MemoRefHead {
        // println!("Context.apply_subject_head({}, {:?}) ", subject_id, head.memo_ids() );
        self.stash.apply_head(&self.slab, head)
    }
    pub (crate) fn get_subject(&self, subject_id: SubjectId) -> Result<Option<Subject>, RetrieveError> {
        let ig = self.root_index.read().unwrap();
        let index = ig.as_ref().ok_or(RetrieveError::IndexNotInitialized)?;

        index.get(&self, subject_id.id)
    }
    /// Retrieve a subject for a known MemoRefHead – ususally used for relationship traversal.
    /// Any relevant context will also be applied when reconstituting the relevant subject to ensure that our consistency model invariants are met
    pub (crate) fn get_subject_with_head(&self,  mut head: MemoRefHead)  -> Result<Subject, RetrieveError> {

        if head.len() == 0 {
            return Err(RetrieveError::InvalidMemoRefHead);
        }

        if let Some(subject_id) = head.subject_id() {
            head.apply( &self.stash.get_head(subject_id), &self.slab );
        }
        
        let subject = Subject::reconstitute(&self, head)?;
        return Ok(subject);

    }
}