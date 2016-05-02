Scouchule
=========

This is a simple Rust program to check for and update date-dependent views. CouchDB doesn't have any notion of dates, and view functions are expected to be deterministic, so you can't write a view that depends on the time a document was posted. This makes scheduling a document to appear in a view in the future impossible.

One solution (the recommended one) is to design your documents so they don't need to do that. This is another solution that is, perhaps, grosser, but more practical. It just bumps the document (ie updates its `_rev`) when it sees it in one query but not another. You can use this to check if the document appears in `_all_docs` but not in your date-dependent view, and bump it if necessary.

It assumes there is some property on your document which is an ISO8601-compatible date. You may need to be careful with your views if there is more than one scheduled document at a time, because it will only touch the most recent one.
