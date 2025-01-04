searchState.loadedDescShard("manyhow", 0, "Proc <strong>m</strong>acro <strong>anyhow</strong>, a combination of ideas from <code>anyhow</code> and …\nMarker trait for <code>proc_macro::TokenStream</code> and …\nExposes <code>ErrorMessage::attachment</code> as a trait to allow …\nInput of an attribute proc-macro\nInput of a derive proc-macro\nAllows emitting errors without returning.\nContains the error value\nThis crates Error type\nA single error message\nInput of a function proc-macro\nAllows to call <code>.join(..)</code> on any <code>impl ToTokensError</code>\nOutput of a macro handler.\nContains the success value\nAn alias for <code>Result</code> suited for use with this crate\nSome utilities on <code>Result&lt;T, impl ToTokensError&gt;</code>\nError that does not expand to any <code>compile_error!</code> and …\nReturns the <code>Range</code><code>&lt;Span&gt;</code> from the start to the end of …\nError that can be converted to a <code>TokenStream</code> required to …\nAttaches an additional message to <code>self</code> reusing the same …\nIf self is error, extend error message\nAttaches an additional message to <code>self</code> reusing the same …\nHandles <code>proc_macro_attribute</code> implementation\nHandles <code>proc_macro_attribute</code> implementation\nExit by returning error, matching <code>anyhow::bail!</code>.\nCreates a new error message at <code>Span::call_site</code> prefer …\nRemoves all emitted errors\nIf self is error, attaches another error\nIf self is error, attaches another error\nIf self is error, attaches another error, closure is only …\nHandles conversion into a Result&lt;TokenStream, Error&gt;.\nHandles <code>proc_macro_derive</code> implementation.\nHandles <code>proc_macro_derive</code> implementation.\nEmits an error\nPush an error to an emitter.\nReturn early with an error, if a condition is not …\nAttaches a new <code>error</code> message to <code>self</code> reusing the same span\nAttaches a new <code>error</code> message to <code>self</code> reusing the same span\nAttaches a new <code>error</code> message to <code>self</code> reusing the same span\nCreates an <code>ErrorMessage</code>, comparable to the <code>anyhow!</code> macro\nReturns the argument unchanged.\nMimics <code>From&lt;impl ToTokensError&gt; for Error</code> implementation to\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nHandles function like <code>proc_macro</code> implementation\nHandles function like <code>proc_macro</code> implementation\nAttaches a new <code>help</code> message to <code>self</code> reusing the same span\nAttaches a new <code>help</code> message to <code>self</code> reusing the same span\nAttaches a new <code>help</code> message to <code>self</code> reusing the same span\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns emitted errors if not <code>Self::is_empty</code>.\nEquivalent to <code>ToTokens::into_token_stream</code>\nEquivalent to <code>ToTokens::into_token_stream</code>\nChecks if any errors were emitted\nJoins two <code>Error</code>s\nAttribute macro to remove boilerplate from proc macro …\nCreates a new error message at the specified span\nCreates an <code>Emitter</code>, this can be used to collect errors …\nAttaches a new <code>note</code> message to <code>self</code> reusing the same span\nAttaches a new <code>note</code> message to <code>self</code> reusing the same span\nAttaches a new <code>note</code> message to <code>self</code> reusing the same span\nPushes an additional <code>Error</code>\nReturns <code>Self::span_range</code> as a single span if possible, …\nReturns <code>Self::span_range</code> as a single span if possible, …\nReturns the <code>Range</code><code>&lt;Span&gt;</code> fully encompasing <code>self</code>\nGet a <code>Range</code><code>&lt;Span&gt;</code> from a type that <strong>either</strong> implements …\nCreates an error message pointing to the complete token …\nEquivalent to <code>ToTokens::to_token_stream</code>\nEquivalent to <code>ToTokens::to_token_stream</code>\nEquivalent to <code>ToTokens::to_tokens</code>\nImplementation of <code>SpanRanged</code><code>for T:</code><code>ToTokens</code>\nAttaches a new <code>warning</code> message to <code>self</code> reusing the same …\nAttaches a new <code>warning</code> message to <code>self</code> reusing the same …\nAttaches a new <code>warning</code> message to <code>self</code> reusing the same …")