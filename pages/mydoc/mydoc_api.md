---
title: Sl-sh Standard Library
tags: [documentation]
keywords: forms, sl-sh, examples, api, standard library
last_updated: March 1, 2021
sidebar: mydoc_sidebar
permalink: mydoc_api.html
toc: false
---

# Sl-sh


## Documentation structure for each form



| <b>form name</b> | <b>type</b> (see: [Type forms](#type-contents)) |
| <b>namespace</b> (fully qualified names are of format namespace::symbol) | <b>usage</b> |

```
example code if exists
```

## Table of Contents

### <a id="char-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Char forms](#char-body)


<a id="root::char-lower-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char-lower``](#root::char-lower), <a id="root::char-upper-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char-upper``](#root::char-upper), <a id="root::char-whitespace?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char-whitespace?``](#root::char-whitespace?)
### <a id="conditional-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Conditional forms](#conditional-body)


<a id="root::<-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``<``](#root::<), <a id="root::<=-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``<=``](#root::<=), <a id="root::=-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``=``](#root::=), <a id="root::>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``>``](#root::>), <a id="root::>=-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``>=``](#root::>=), <a id="root::and-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``and``](#root::and), <a id="root::cond-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cond``](#root::cond), <a id="root::if-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``if``](#root::if), <a id="root::match-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``match``](#root::match), <a id="root::not-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``not``](#root::not), <a id="root::null-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``null``](#root::null), <a id="root::or-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``or``](#root::or), <a id="root::when-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``when``](#root::when)
### <a id="core-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Core forms](#core-body)


<a id="root::*collection-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*collection-src*``](#root::*collection-src*), <a id="root::*core-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*core-src*``](#root::*core-src*), <a id="root::*getopts-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*getopts-src*``](#root::*getopts-src*), <a id="root::*iterator-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*iterator-src*``](#root::*iterator-src*), <a id="root::*lib-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*lib-src*``](#root::*lib-src*), <a id="root::*seq-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*seq-src*``](#root::*seq-src*), <a id="root::*shell-read-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*shell-read-src*``](#root::*shell-read-src*), <a id="root::*shell-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*shell-src*``](#root::*shell-src*), <a id="root::*slsh-std-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*slsh-std-src*``](#root::*slsh-std-src*), <a id="root::*slshrc-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*slshrc-src*``](#root::*slshrc-src*), <a id="root::*struct-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*struct-src*``](#root::*struct-src*), <a id="root::*test-src*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*test-src*``](#root::*test-src*), <a id="root::and-let*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``and-let*``](#root::and-let*), <a id="root::apply-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``apply``](#root::apply), <a id="root::back-quote-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``back-quote``](#root::back-quote), <a id="root::block-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``block``](#root::block), <a id="root::dec!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dec!``](#root::dec!), <a id="root::def-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``def``](#root::def), <a id="root::def?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``def?``](#root::def?), <a id="root::defmacro-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``defmacro``](#root::defmacro), <a id="root::defn-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``defn``](#root::defn), <a id="root::do-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``do``](#root::do), <a id="root::doc-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``doc``](#root::doc), <a id="root::doc-raw-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``doc-raw``](#root::doc-raw), <a id="root::dotimes-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dotimes``](#root::dotimes), <a id="root::dotimes-i-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dotimes-i``](#root::dotimes-i), <a id="root::dyn-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dyn``](#root::dyn), <a id="root::eprint-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``eprint``](#root::eprint), <a id="root::eprintln-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``eprintln``](#root::eprintln), <a id="root::err-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err``](#root::err), <a id="root::error-stack-off-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``error-stack-off``](#root::error-stack-off), <a id="root::error-stack-on-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``error-stack-on``](#root::error-stack-on), <a id="root::eval-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``eval``](#root::eval), <a id="root::expand-macro-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``expand-macro``](#root::expand-macro), <a id="root::expand-macro-all-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``expand-macro-all``](#root::expand-macro-all), <a id="root::expand-macro1-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``expand-macro1``](#root::expand-macro1), <a id="root::fn-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fn``](#root::fn), <a id="root::format-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``format``](#root::format), <a id="root::gensym-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``gensym``](#root::gensym), <a id="root::get-error-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-error``](#root::get-error), <a id="root::identity-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``identity``](#root::identity), <a id="root::inc!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``inc!``](#root::inc!), <a id="root::intern-stats-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``intern-stats``](#root::intern-stats), <a id="root::len0?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``len0?``](#root::len0?), <a id="root::len>0?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``len>0?``](#root::len>0?), <a id="root::length-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``length``](#root::length), <a id="root::let-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``let``](#root::let), <a id="root::let*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``let*``](#root::let*), <a id="root::loop-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``loop``](#root::loop), <a id="root::macro-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``macro``](#root::macro), <a id="root::maybe-docstring?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``maybe-docstring?``](#root::maybe-docstring?), <a id="root::meta-add-tags-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-add-tags``](#root::meta-add-tags), <a id="root::meta-column-no-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-column-no``](#root::meta-column-no), <a id="root::meta-file-name-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-file-name``](#root::meta-file-name), <a id="root::meta-line-no-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-line-no``](#root::meta-line-no), <a id="root::meta-tag?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-tag?``](#root::meta-tag?), <a id="root::nsubstitute!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``nsubstitute!``](#root::nsubstitute!), <a id="root::occurs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``occurs``](#root::occurs), <a id="root::print-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``print``](#root::print), <a id="root::print-error-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``print-error``](#root::print-error), <a id="root::println-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``println``](#root::println), <a id="root::quote-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``quote``](#root::quote), <a id="root::reader-macro-dot-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reader-macro-dot``](#root::reader-macro-dot), <a id="root::recur-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``recur``](#root::recur), <a id="root::ref-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ref``](#root::ref), <a id="root::return-from-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``return-from``](#root::return-from), <a id="root::set!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``set!``](#root::set!), <a id="root::substitute-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``substitute``](#root::substitute), <a id="root::undef-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``undef``](#root::undef), <a id="root::unwind-protect-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unwind-protect``](#root::unwind-protect), <a id="root::values-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values``](#root::values), <a id="root::values-length-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values-length``](#root::values-length), <a id="root::values-nth-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values-nth``](#root::values-nth), <a id="root::var-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``var``](#root::var)
### <a id="file-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[File forms](#file-body)


<a id="root::cd-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cd``](#root::cd), <a id="root::close-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``close``](#root::close), <a id="root::flush-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``flush``](#root::flush), <a id="root::fs-dir?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fs-dir?``](#root::fs-dir?), <a id="root::fs-exists?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fs-exists?``](#root::fs-exists?), <a id="root::fs-file?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fs-file?``](#root::fs-file?), <a id="root::glob-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``glob``](#root::glob), <a id="root::open-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``open``](#root::open), <a id="root::read-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``read``](#root::read), <a id="root::read-all-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``read-all``](#root::read-all), <a id="root::read-line-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``read-line``](#root::read-line), <a id="root::write-line-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``write-line``](#root::write-line), <a id="root::write-string-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``write-string``](#root::write-string)
### <a id="globals-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Globals forms](#globals-body)


<a id="root::*last-command*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*last-command*``](#root::*last-command*), <a id="root::*last-status*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*last-status*``](#root::*last-status*), <a id="root::*repl-settings*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*repl-settings*``](#root::*repl-settings*), <a id="root::*std-lib-namespaces*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*std-lib-namespaces*``](#root::*std-lib-namespaces*), <a id="root::*std-lib-syms-hash*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*std-lib-syms-hash*``](#root::*std-lib-syms-hash*)
### <a id="hashmap-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Hashmap forms](#hashmap-body)


<a id="root::hash-clear!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-clear!``](#root::hash-clear!), <a id="root::hash-get-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-get``](#root::hash-get), <a id="root::hash-haskey-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-haskey``](#root::hash-haskey), <a id="root::hash-keys-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-keys``](#root::hash-keys), <a id="root::hash-remove!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-remove!``](#root::hash-remove!), <a id="root::hash-set!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-set!``](#root::hash-set!), <a id="root::make-hash-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``make-hash``](#root::make-hash)
### <a id="iterator-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Iterator forms](#iterator-body)


<a id="iterator::append-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``append``](#iterator::append), <a id="iterator::append-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``append-iter``](#iterator::append-iter), <a id="iterator::append-to!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``append-to!``](#iterator::append-to!), <a id="iterator::collect-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect``](#iterator::collect), <a id="iterator::collect-str-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect-str``](#iterator::collect-str), <a id="iterator::collect-vec-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect-vec``](#iterator::collect-vec), <a id="iterator::double-ended-iter?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``double-ended-iter?``](#iterator::double-ended-iter?), <a id="iterator::double-ended-iterator-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``double-ended-iterator``](#iterator::double-ended-iterator), <a id="iterator::empty?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``empty?``](#iterator::empty?), <a id="iterator::file-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``file-iter``](#iterator::file-iter), <a id="iterator::filter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``filter``](#iterator::filter), <a id="iterator::filter-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``filter-iter``](#iterator::filter-iter), <a id="iterator::for-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``for``](#iterator::for), <a id="iterator::for-i-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``for-i``](#iterator::for-i), <a id="iterator::interleave-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``interleave``](#iterator::interleave), <a id="iterator::interleave-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``interleave-iter``](#iterator::interleave-iter), <a id="iterator::iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``iter``](#iterator::iter), <a id="iterator::iter?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``iter?``](#iterator::iter?), <a id="iterator::iterator-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``iterator``](#iterator::iterator), <a id="iterator::list-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``list-iter``](#iterator::list-iter), <a id="iterator::map-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``map``](#iterator::map), <a id="iterator::map-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``map-iter``](#iterator::map-iter), <a id="iterator::meld-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meld``](#iterator::meld), <a id="iterator::meld-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meld-iter``](#iterator::meld-iter), <a id="iterator::next!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``next!``](#iterator::next!), <a id="iterator::nth-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``nth``](#iterator::nth), <a id="iterator::range-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``range``](#iterator::range), <a id="iterator::range-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``range-iter``](#iterator::range-iter), <a id="iterator::reduce-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reduce``](#iterator::reduce), <a id="iterator::reduce-times-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reduce-times``](#iterator::reduce-times), <a id="iterator::repeat-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``repeat``](#iterator::repeat), <a id="iterator::repeat-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``repeat-iter``](#iterator::repeat-iter), <a id="iterator::reverse-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reverse``](#iterator::reverse), <a id="iterator::reverse-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reverse-iter``](#iterator::reverse-iter), <a id="iterator::slice-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``slice``](#iterator::slice), <a id="iterator::slice-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``slice-iter``](#iterator::slice-iter), <a id="iterator::string-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``string-iter``](#iterator::string-iter), <a id="iterator::take-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``take``](#iterator::take), <a id="iterator::take-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``take-iter``](#iterator::take-iter), <a id="iterator::vec-iter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-iter``](#iterator::vec-iter)
### <a id="logger-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Logger forms](#logger-body)


<a id="root::logger-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``logger``](#root::logger)
### <a id="math-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Math forms](#math-body)


<a id="root::%-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``%``](#root::%), <a id="root::*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*``](#root::*), <a id="math::*euler*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*euler*``](#math::*euler*), <a id="math::*pi*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*pi*``](#math::*pi*), <a id="root::+-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``+``](#root::+), <a id="root::--contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``-``](#root::-), <a id="root::/-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``/``](#root::/), <a id="math::2pow-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``2pow``](#math::2pow), <a id="math::abs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``abs``](#math::abs), <a id="math::arccos-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``arccos``](#math::arccos), <a id="math::arcsin-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``arcsin``](#math::arcsin), <a id="math::arctan-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``arctan``](#math::arctan), <a id="math::ceil-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ceil``](#math::ceil), <a id="math::cos-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cos``](#math::cos), <a id="math::exp-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``exp``](#math::exp), <a id="math::floor-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``floor``](#math::floor), <a id="math::fract-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fract``](#math::fract), <a id="math::lne-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``lne``](#math::lne), <a id="math::log-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``log``](#math::log), <a id="math::log2-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``log2``](#math::log2), <a id="math::pow-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pow``](#math::pow), <a id="math::round-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``round``](#math::round), <a id="math::sin-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sin``](#math::sin), <a id="math::sqrt-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sqrt``](#math::sqrt), <a id="math::tan-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``tan``](#math::tan), <a id="math::to-degrees-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``to-degrees``](#math::to-degrees), <a id="math::to-radians-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``to-radians``](#math::to-radians)
### <a id="namespace-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Namespace forms](#namespace-body)


<a id="root::ns-auto-export-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-auto-export``](#root::ns-auto-export), <a id="root::ns-create-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-create``](#root::ns-create), <a id="root::ns-enter-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-enter``](#root::ns-enter), <a id="root::ns-exists?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-exists?``](#root::ns-exists?), <a id="root::ns-export-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-export``](#root::ns-export), <a id="root::ns-import-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-import``](#root::ns-import), <a id="root::ns-list-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-list``](#root::ns-list), <a id="root::ns-pop-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-pop``](#root::ns-pop), <a id="root::ns-push-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-push``](#root::ns-push), <a id="root::ns-symbols-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-symbols``](#root::ns-symbols)
### <a id="pair-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Pair forms](#pair-body)


<a id="root::car-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``car``](#root::car), <a id="root::cdr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdr``](#root::cdr), <a id="root::join-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``join``](#root::join), <a id="root::list-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``list``](#root::list), <a id="root::xar!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``xar!``](#root::xar!), <a id="root::xdr!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``xdr!``](#root::xdr!)
### <a id="pair-ext-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Pair-ext forms](#pair-ext-body)


<a id="root::caaar-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caaar``](#root::caaar), <a id="root::caadr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caadr``](#root::caadr), <a id="root::caar-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caar``](#root::caar), <a id="root::cadar-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cadar``](#root::cadar), <a id="root::cadddr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cadddr``](#root::cadddr), <a id="root::caddr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caddr``](#root::caddr), <a id="root::cadr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cadr``](#root::cadr), <a id="root::cdaar-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdaar``](#root::cdaar), <a id="root::cdadr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdadr``](#root::cdadr), <a id="root::cdar-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdar``](#root::cdar), <a id="root::cddar-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cddar``](#root::cddar), <a id="root::cdddr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdddr``](#root::cdddr), <a id="root::cddr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cddr``](#root::cddr)
### <a id="random-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Random forms](#random-body)


<a id="root::probool-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``probool``](#root::probool), <a id="root::random-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``random``](#root::random), <a id="root::random-str-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``random-str``](#root::random-str)
### <a id="root-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Root forms](#root-body)


<a id="root::*read-table*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*read-table*``](#root::*read-table*), <a id="root::*read-table-terminal*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*read-table-terminal*``](#root::*read-table-terminal*), <a id="root::*string-read-table*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*string-read-table*``](#root::*string-read-table*)
### <a id="scripting-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Scripting forms](#scripting-body)


<a id="root::*load-path*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*load-path*``](#root::*load-path*), <a id="root::load-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``load``](#root::load), <a id="shell::mkli-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``mkli``](#shell::mkli)
### <a id="sequence-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Sequence forms](#sequence-body)


<a id="root::butlast-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``butlast``](#root::butlast), <a id="root::collect-copy-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect-copy``](#root::collect-copy), <a id="root::empty-seq?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``empty-seq?``](#root::empty-seq?), <a id="root::first-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``first``](#root::first), <a id="root::in?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``in?``](#root::in?), <a id="root::last-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``last``](#root::last), <a id="root::non-empty-seq?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``non-empty-seq?``](#root::non-empty-seq?), <a id="root::qsort-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``qsort``](#root::qsort), <a id="root::rest-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``rest``](#root::rest), <a id="root::seq-for-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``seq-for``](#root::seq-for), <a id="root::seq?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``seq?``](#root::seq?), <a id="root::setnth!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``setnth!``](#root::setnth!)
### <a id="shell-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Shell forms](#shell-body)


<a id="root::*stderr*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*stderr*``](#root::*stderr*), <a id="root::*stdin*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*stdin*``](#root::*stdin*), <a id="root::*stdout*-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*stdout*``](#root::*stdout*), <a id="shell::alias-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``alias``](#shell::alias), <a id="shell::alias?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``alias?``](#shell::alias?), <a id="shell::bg-color-rgb-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``bg-color-rgb``](#shell::bg-color-rgb), <a id="shell::clear-dirs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``clear-dirs``](#shell::clear-dirs), <a id="shell::dirs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dirs``](#shell::dirs), <a id="root::epoch-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``epoch``](#root::epoch), <a id="shell::err>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err>``](#shell::err>), <a id="shell::err>>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err>>``](#shell::err>>), <a id="shell::err>null-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err>null``](#shell::err>null), <a id="shell::fc-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fc``](#shell::fc), <a id="shell::fg-color-rgb-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fg-color-rgb``](#shell::fg-color-rgb), <a id="shell::get-dirs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-dirs``](#shell::get-dirs), <a id="shell::getopts-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``getopts``](#shell::getopts), <a id="shell::getopts-help-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``getopts-help``](#shell::getopts-help), <a id="root::history-context-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-context``](#root::history-context), <a id="root::history-empty?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-empty?``](#root::history-empty?), <a id="root::history-length-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-length``](#root::history-length), <a id="root::history-nth-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-nth``](#root::history-nth), <a id="root::history-push-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-push``](#root::history-push), <a id="root::history-push-throwaway-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-push-throwaway``](#root::history-push-throwaway), <a id="shell::let-env-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``let-env``](#shell::let-env), <a id="shell::out-err>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out-err>``](#shell::out-err>), <a id="shell::out-err>>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out-err>>``](#shell::out-err>>), <a id="shell::out-err>null-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out-err>null``](#shell::out-err>null), <a id="shell::out>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out>``](#shell::out>), <a id="shell::out>>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out>>``](#shell::out>>), <a id="shell::out>null-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out>null``](#shell::out>null), <a id="shell::popd-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``popd``](#shell::popd), <a id="root::prompt-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``prompt``](#root::prompt), <a id="shell::pushd-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pushd``](#shell::pushd), <a id="shell::register-alias-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``register-alias``](#shell::register-alias), <a id="shell::set-dirs-max-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``set-dirs-max``](#shell::set-dirs-max), <a id="shell::syntax-off-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``syntax-off``](#shell::syntax-off), <a id="shell::syntax-on-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``syntax-on``](#shell::syntax-on), <a id="shell::sys-command?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sys-command?``](#shell::sys-command?), <a id="shell::temp-dir-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``temp-dir``](#shell::temp-dir), <a id="shell::timer-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``timer``](#shell::timer), <a id="shell::unalias-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unalias``](#shell::unalias), <a id="shell::unregister-alias-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unregister-alias``](#shell::unregister-alias), <a id="root::version-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``version``](#root::version)
### <a id="stats-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Stats forms](#stats-body)


<a id="stats::first-quartile-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``first-quartile``](#stats::first-quartile), <a id="stats::max-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``max``](#stats::max), <a id="stats::mean-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``mean``](#stats::mean), <a id="stats::median-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``median``](#stats::median), <a id="stats::min-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``min``](#stats::min), <a id="stats::mode-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``mode``](#stats::mode), <a id="stats::std-dev-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``std-dev``](#stats::std-dev), <a id="stats::summary-stats-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``summary-stats``](#stats::summary-stats), <a id="stats::third-quartile-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``third-quartile``](#stats::third-quartile)
### <a id="string-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[String forms](#string-body)


<a id="root::char->int-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char->int``](#root::char->int), <a id="root::codepoints-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``codepoints``](#root::codepoints), <a id="root::do-unstr-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``do-unstr``](#root::do-unstr), <a id="root::str-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str``](#root::str), <a id="root::str-append-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-append``](#root::str-append), <a id="root::str-bytes-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-bytes``](#root::str-bytes), <a id="root::str-cat-list-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-cat-list``](#root::str-cat-list), <a id="root::str-clear!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-clear!``](#root::str-clear!), <a id="root::str-contains-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-contains``](#root::str-contains), <a id="root::str-empty?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-empty?``](#root::str-empty?), <a id="root::str-iter-empty?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-empty?``](#root::str-iter-empty?), <a id="root::str-iter-next!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-next!``](#root::str-iter-next!), <a id="root::str-iter-peek-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-peek``](#root::str-iter-peek), <a id="root::str-iter-start-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-start``](#root::str-iter-start), <a id="root::str-lower-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-lower``](#root::str-lower), <a id="root::str-ltrim-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-ltrim``](#root::str-ltrim), <a id="root::str-map-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-map``](#root::str-map), <a id="root::str-nth-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-nth``](#root::str-nth), <a id="root::str-push!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-push!``](#root::str-push!), <a id="root::str-replace-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-replace``](#root::str-replace), <a id="root::str-rsplit-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-rsplit``](#root::str-rsplit), <a id="root::str-rsplitn-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-rsplitn``](#root::str-rsplitn), <a id="root::str-rtrim-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-rtrim``](#root::str-rtrim), <a id="root::str-split-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-split``](#root::str-split), <a id="root::str-splitn-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-splitn``](#root::str-splitn), <a id="root::str-starts-with-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-starts-with``](#root::str-starts-with), <a id="root::str-sub-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-sub``](#root::str-sub), <a id="root::str-trim-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-trim``](#root::str-trim), <a id="root::str-upper-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-upper``](#root::str-upper), <a id="root::with-padding-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``with-padding``](#root::with-padding)
### <a id="struct-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Struct forms](#struct-body)


<a id="struct::defstruct-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``defstruct``](#struct::defstruct), <a id="struct::deftrait-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``deftrait``](#struct::deftrait)
### <a id="system-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[System forms](#system-body)


<a id="root::bg-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``bg``](#root::bg), <a id="root::exit-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``exit``](#root::exit), <a id="root::export-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``export``](#root::export), <a id="root::fg-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fg``](#root::fg), <a id="root::fork-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fork``](#root::fork), <a id="root::get-env-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-env``](#root::get-env), <a id="root::get-pid-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-pid``](#root::get-pid), <a id="root::jobs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``jobs``](#root::jobs), <a id="root::pid-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pid``](#root::pid), <a id="root::pipe-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pipe``](#root::pipe), <a id="root::reap-jobs-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reap-jobs``](#root::reap-jobs), <a id="root::sleep-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sleep``](#root::sleep), <a id="root::syscall-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``syscall``](#root::syscall), <a id="root::time-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``time``](#root::time), <a id="root::unexport-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unexport``](#root::unexport), <a id="root::wait-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``wait``](#root::wait)
### <a id="test-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Test forms](#test-body)


<a id="test::assert-equal-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-equal``](#test::assert-equal), <a id="test::assert-error-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-error``](#test::assert-error), <a id="test::assert-error-msg-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-error-msg``](#test::assert-error-msg), <a id="test::assert-false-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-false``](#test::assert-false), <a id="test::assert-not-equal-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-not-equal``](#test::assert-not-equal), <a id="test::assert-true-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-true``](#test::assert-true), <a id="test::run-example-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``run-example``](#test::run-example)
### <a id="threading-macros-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Threading-macros forms](#threading-macros-body)


<a id="root::->-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``->``](#root::->), <a id="root::->>-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``->>``](#root::->>), <a id="root::chain-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``chain``](#root::chain), <a id="root::chain-and-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``chain-and``](#root::chain-and), <a id="root::chain-when-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``chain-when``](#root::chain-when)
### <a id="type-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Type forms](#type-body)


<a id="root::boolean?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``boolean?``](#root::boolean?), <a id="root::builtin?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``builtin?``](#root::builtin?), <a id="root::char?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char?``](#root::char?), <a id="root::false?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``false?``](#root::false?), <a id="root::falsey?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``falsey?``](#root::falsey?), <a id="root::file?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``file?``](#root::file?), <a id="root::float->int-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``float->int``](#root::float->int), <a id="root::float?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``float?``](#root::float?), <a id="root::func?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``func?``](#root::func?), <a id="root::hash?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash?``](#root::hash?), <a id="root::int->float-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``int->float``](#root::int->float), <a id="root::int?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``int?``](#root::int?), <a id="root::lambda?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``lambda?``](#root::lambda?), <a id="root::list?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``list?``](#root::list?), <a id="root::macro?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``macro?``](#root::macro?), <a id="root::nil?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``nil?``](#root::nil?), <a id="root::none?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``none?``](#root::none?), <a id="root::pair?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pair?``](#root::pair?), <a id="root::process?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``process?``](#root::process?), <a id="root::some?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``some?``](#root::some?), <a id="root::str->float-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str->float``](#root::str->float), <a id="root::str->int-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str->int``](#root::str->int), <a id="root::string?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``string?``](#root::string?), <a id="root::sym-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sym``](#root::sym), <a id="root::sym->str-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sym->str``](#root::sym->str), <a id="root::symbol?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``symbol?``](#root::symbol?), <a id="root::true?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``true?``](#root::true?), <a id="root::type-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``type``](#root::type), <a id="root::values?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values?``](#root::values?), <a id="root::vec?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec?``](#root::vec?)
### <a id="vector-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Vector forms](#vector-body)


<a id="root::make-vec-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``make-vec``](#root::make-vec), <a id="root::vec-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec``](#root::vec), <a id="root::vec-clear!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-clear!``](#root::vec-clear!), <a id="root::vec-empty?-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-empty?``](#root::vec-empty?), <a id="root::vec-insert!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-insert!``](#root::vec-insert!), <a id="root::vec-nth-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-nth``](#root::vec-nth), <a id="root::vec-pop!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-pop!``](#root::vec-pop!), <a id="root::vec-push!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-push!``](#root::vec-push!), <a id="root::vec-remove!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-remove!``](#root::vec-remove!), <a id="root::vec-set!-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-set!``](#root::vec-set!), <a id="root::vec-slice-contents" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-slice``](#root::vec-slice)

## Documentation

### <a id="char-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Char forms](#char-contents)



| <a id="root::char-lower" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char-lower``](#root::char-lower-contents) | Type: Function |
 ``root::char-lower`` | ``Usage: (char-lower char) -> char`` |

<span style="padding-left: 5px">Get lower case (utf) character for a character.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal #\a (char-lower #\A))<br>
(test::assert-equal #\a (char-lower #\a))<br>
(test::assert-not-equal #\a (char-lower #\Z))<br>
(test::assert-equal #\λ (char-lower #\Λ))<br>
(test::assert-equal #\λ (char-lower #\λ))<br>
<br>
</code>
</details>
<br>


| <a id="root::char-upper" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char-upper``](#root::char-upper-contents) | Type: Function |
 ``root::char-upper`` | ``Usage: (char-upper char) -> char`` |

<span style="padding-left: 5px">Get upper case (utf) character for a character.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal #\A (char-upper #\A))<br>
(test::assert-equal #\A (char-upper #\a))<br>
(test::assert-not-equal #\A (char-upper #\Z))<br>
(test::assert-equal #\Λ (char-upper #\λ))<br>
(test::assert-equal #\Λ (char-upper #\Λ))<br>
<br>
</code>
</details>
<br>


| <a id="root::char-whitespace?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char-whitespace?``](#root::char-whitespace?-contents) | Type: Function |
 ``root::char-whitespace?`` | ``Usage: (char-whitespace? char) -> t/nil`` |

<span style="padding-left: 5px">Returns true if a character is whitespace, false/nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (char-whitespace? #\ ))<br>
(test::assert-true (char-whitespace? #\tab))<br>
(test::assert-false (char-whitespace? #\s))<br>
<br>
</code>
</details>
<br>
### <a id="conditional-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Conditional forms](#conditional-contents)



| <a id="root::<" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``<``](#root::<-contents) | Type: Function |
 ``root::<`` | ``Usage: (< val0 ... valN)`` |

<span style="padding-left: 5px">Less than.  Works for int, float or string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (< 1 2))<br>
(test::assert-true (< 1 2 3 4))<br>
(test::assert-false (< 2 2))<br>
(test::assert-false (< 2 2 2))<br>
(test::assert-false (< 2 2 3))<br>
(test::assert-true (< 1.0 2.0))<br>
(test::assert-false (< 2.0 2.0))<br>
(test::assert-false (< 2.0 2.0 2.0))<br>
(test::assert-false (< 2.0 2.0 3.0))<br>
(test::assert-false (< 2.1 2.0 3.0))<br>
(test::assert-false (< 2 1))<br>
(test::assert-false (< 3 2 3))<br>
(test::assert-true (< "aaa" "aab"))<br>
(test::assert-false (< "aaa" "aaa"))<br>
(test::assert-true (< "aaa" "aab" "ccc"))<br>
(test::assert-false (< "baa" "aab"))<br>
<br>
</code>
</details>
<br>


| <a id="root::<=" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``<=``](#root::<=-contents) | Type: Function |
 ``root::<=`` | ``Usage: (<= val0 ... valN)`` |

<span style="padding-left: 5px">Less than or equal.  Works for int, float or string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (<= 1 2))<br>
(test::assert-true (<= 2 2))<br>
(test::assert-true (<= 2 2 2))<br>
(test::assert-true (<= 2 2 3))<br>
(test::assert-true (<= 1.0 2.0))<br>
(test::assert-true (<= 2.0 2.0))<br>
(test::assert-true (<= 2.0 2.0 2.0))<br>
(test::assert-true (<= 2.0 2.0 3.0))<br>
(test::assert-false (<= 2.1 2.0 3.0))<br>
(test::assert-false (<= 2 1))<br>
(test::assert-false (<= 3 2 3))<br>
(test::assert-true (<= "aaa" "aab"))<br>
(test::assert-true (<= "aaa" "aaa"))<br>
(test::assert-true (<= "aaa" "aab" "ccc"))<br>
(test::assert-false (<= "baa" "aab"))<br>
<br>
</code>
</details>
<br>


| <a id="root::=" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``=``](#root::=-contents) | Type: Function |
 ``root::=`` | ``Usage: (= val0 ... valN)`` |

<span style="padding-left: 5px">Equals.  Works for int, float or string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (= 1 2))<br>
(test::assert-true (= 2 2))<br>
(test::assert-true (= 2 2 2))<br>
(test::assert-false (= 3 2 2))<br>
(test::assert-false (= 3.0 2.0))<br>
(test::assert-true (= 2.0 2.0))<br>
(test::assert-true (= 2.0 2.0 2.0))<br>
(test::assert-false (= 3.0 2.0 2.0))<br>
(test::assert-false (= 2.1 2.0 3.0))<br>
(test::assert-false (= 2 1))<br>
(test::assert-false (= 3 2 1))<br>
(test::assert-false (= 1.1 1.0))<br>
(test::assert-true (= 1.1 1.1))<br>
(test::assert-false (= 3 2 3))<br>
(test::assert-false (= "aab" "aaa"))<br>
(test::assert-true (= "aaa" "aaa"))<br>
(test::assert-true (= "aaa" "aaa" "aaa"))<br>
(test::assert-false (= "aaa" "aaaa" "aaa"))<br>
(test::assert-false (= "ccc" "aab" "aaa"))<br>
(test::assert-false (= "aaa" "aab"))<br>
<br>
</code>
</details>
<br>


| <a id="root::>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``>``](#root::>-contents) | Type: Function |
 ``root::>`` | ``Usage: (> val0 ... valN)`` |

<span style="padding-left: 5px">Greater than.  Works for int, float or string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (> 1 2))<br>
(test::assert-false (> 2 2))<br>
(test::assert-false (> 2 2 2))<br>
(test::assert-false (> 3 2 2))<br>
(test::assert-true (> 3.0 2.0))<br>
(test::assert-false (> 2.0 2.0))<br>
(test::assert-false (> 2.0 2.0 2.0))<br>
(test::assert-false (> 3.0 2.0 2.0))<br>
(test::assert-false (> 2.1 2.0 3.0))<br>
(test::assert-true (> 2 1))<br>
(test::assert-true (> 3 2 1))<br>
(test::assert-true (> 1.1 1.0))<br>
(test::assert-false (> 3 2 3))<br>
(test::assert-true (> "aab" "aaa"))<br>
(test::assert-false (> "aaa" "aaa"))<br>
(test::assert-true (> "ccc" "aab" "aaa"))<br>
(test::assert-false (> "aaa" "aab"))<br>
<br>
</code>
</details>
<br>


| <a id="root::>=" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``>=``](#root::>=-contents) | Type: Function |
 ``root::>=`` | ``Usage: (>= val0 ... valN)`` |

<span style="padding-left: 5px">Greater than or equal.  Works for int, float or string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (>= 1 2))<br>
(test::assert-true (>= 2 2))<br>
(test::assert-true (>= 2 2 2))<br>
(test::assert-true (>= 3 2 2))<br>
(test::assert-true (>= 3.0 2.0))<br>
(test::assert-true (>= 2.0 2.0))<br>
(test::assert-true (>= 2.0 2.0 2.0))<br>
(test::assert-true (>= 3.0 2.0 2.0))<br>
(test::assert-false (>= 2.1 2.0 3.0))<br>
(test::assert-true (>= 2 1))<br>
(test::assert-true (>= 1.1 1.0))<br>
(test::assert-false (>= 3 2 3))<br>
(test::assert-true (>= "aab" "aaa"))<br>
(test::assert-true (>= "aaa" "aaa"))<br>
(test::assert-true (>= "ccc" "aab" "aaa"))<br>
(test::assert-false (>= "aaa" "aab"))<br>
<br>
</code>
</details>
<br>


| <a id="root::and" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``and``](#root::and-contents) | Type: SpecialForm |
 ``root::and`` | ``Usage: (and exp0 ... expN) -> [false(#f) or expN result]`` |

<span style="padding-left: 5px">Evaluates each form until one produces nil or false(#f), produces false(#f) if
any form is nil/#f or the result of the last expression.
The and form will stop evaluating when the first expression produces nil/#f.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal #f (and nil (err "and- can not happen")))<br>
(test::assert-equal #f (and #f (err "and- can not happen")))<br>
(test::assert-equal "and- done" (and #t "and- done"))<br>
(test::assert-equal "and- done" (and #t #t "and- done"))<br>
(test::assert-equal 6 (and #t #t (+ 1 2 3)))<br>
(test::assert-equal 6 (and (/ 10 5) (* 5 2) (+ 1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cond" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cond``](#root::cond-contents) | Type: Macro |
 ``root::cond`` | ``Usage: (cond ((test form*)*) -> result`` |

<span style="padding-left: 5px">Evaluate each test in order.  If it is true then evaluate the form(s) in an
implicit do and return the result.  Stop evaluting at the first true test.
Return nil if no conditions are true.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defn select-option (a)<br>
(cond ((= a 1) "opt-one")<br>
((= a 2) (set! b 5) "opt-two")<br>
((= a 3) (str "opt" "-three"))))<br>
(defn select-option-def (a)<br>
(cond ((= a 1) "opt-one")<br>
((= a 2) "opt-two")<br>
((= a 3) (str "opt" "-three"))<br>
(#t "default")))<br>
(def b 0)<br>
(assert-equal "opt-one" (select-option 1))<br>
(assert-equal b 0)<br>
(assert-equal "opt-two" (select-option 2))<br>
(assert-equal b 5)<br>
(assert-equal "opt-three" (select-option 3))<br>
(assert-equal nil (select-option 4))<br>
(assert-equal "opt-one" (select-option-def 1))<br>
(assert-equal "opt-two" (select-option-def 2))<br>
(assert-equal "opt-three" (select-option-def 3))<br>
(assert-equal "default" (select-option-def 4))<br>
<br>
</code>
</details>
<br>


| <a id="root::if" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``if``](#root::if-contents) | Type: SpecialForm |
 ``root::if`` | ``Usage: (if p1 a1 p2 a2 ... pn an?) -> [evaled form result]`` |

<span style="padding-left: 5px">If conditional.  Will evaluate p1 and if true (i.e. not nil or false) then
return the evaluation of a1, if falsy(i.e. nil or false) evaluate p2 and so on.
On an odd number of arguments (an is missing) then evaluate and return pn.
Return false(#f) if no predicate is true.  This degenerates into the traditional
(if predicate then-form else-form).
NOTE: Both nil and false(#f) are 'falsy' for the purposes of if.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-if-one<br>
(if #t "ONE TRUE" "ONE FALSE"))<br>
(def test-if-two<br>
(if nil "TWO TRUE" "TWO FALSE"))<br>
(def test-if-three<br>
(if #f "THREE TRUE" "THREE FALSE"))<br>
(test::assert-equal "ONE TRUE" test-if-one)<br>
(test::assert-equal "TWO FALSE" test-if-two)<br>
(test::assert-equal "THREE FALSE" test-if-three)<br>
(def test-if-one2<br>
(if #t "ONE2 TRUE"))<br>
(def test-if-two2<br>
(if nil "TWO2 TRUE"))<br>
(def test-if-three2<br>
(if #f "THREE2 TRUE"))<br>
(test::assert-equal "ONE2 TRUE" test-if-one2)<br>
(test::assert-equal #f test-if-two2)<br>
(test::assert-equal #f test-if-three2)<br>
(def test-if-one2<br>
(if nil "ONE FALSE" #t "ONE TRUE" #t "ONE TRUE2"))<br>
(def test-if-two2<br>
(if nil "TWO TRUE" #f "TWO FALSE" #t "TWO TRUE2"))<br>
(def test-if-three2<br>
(if #f "THREE TRUE" nil "THREE FALSE" "THREE DEFAULT"))<br>
(test::assert-equal "ONE TRUE" test-if-one2)<br>
(test::assert-equal "TWO TRUE2" test-if-two2)<br>
(test::assert-equal "THREE DEFAULT" test-if-three2)<br>
(test::assert-equal nil (if nil))<br>
(test::assert-equal #f (if nil #t nil #t nil t))<br>
<br>
</code>
</details>
<br>


| <a id="root::match" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``match``](#root::match-contents) | Type: Macro |
 ``root::match`` | ``Usage: (match condition (value form*)*) -> result`` |

<span style="padding-left: 5px">Evaluate condition and look for matching value in each branch of type
(value form*). Form(s) will be wrapped in an implicit do. Use nil to take
action if no match (encouraged!).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defn select-option (a)<br>
(match a (1 "opt-one")<br>
(2 (set! b 5) "opt-two")<br>
(3 (str "opt" "-three"))))<br>
(defn select-option-def (a)<br>
(match a (1 "opt-one")<br>
(2 "opt-two")<br>
(3 (str "opt" "-three"))<br>
(nil "default")))<br>
(def b 0)<br>
(assert-equal b 0)<br>
(assert-equal "opt-one" (select-option 1))<br>
(assert-equal "opt-two" (select-option 2))<br>
(assert-equal b 5)<br>
(assert-equal "opt-three" (select-option 3))<br>
(assert-equal #f (select-option 4))<br>
(assert-equal "opt-one" (select-option-def 1))<br>
(assert-equal "opt-two" (select-option-def 2))<br>
(assert-equal "opt-three" (select-option-def 3))<br>
(assert-equal "default" (select-option-def 4))<br>
<br>
</code>
</details>
<br>


| <a id="root::not" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``not``](#root::not-contents) | Type: Function |
 ``root::not`` | ``Usage: (not expression)`` |

<span style="padding-left: 5px">Return true(#t) if expression is nil, false(#f) otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (not nil))<br>
(test::assert-false (not 10))<br>
(test::assert-false (not #t))<br>
(test::assert-false (not (+ 1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::null" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``null``](#root::null-contents) | Type: Function |
 ``root::null`` | ``Usage: (null expression)`` |

<span style="padding-left: 5px">Return true(#t) if expression is nil (null).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (null nil))<br>
(test::assert-false (null 10))<br>
(test::assert-false (null #t))<br>
(test::assert-false (null (+ 1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::or" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``or``](#root::or-contents) | Type: SpecialForm |
 ``root::or`` | ``Usage: (or exp0 ... expN) -> [false(#f) or first non nil expression]`` |

<span style="padding-left: 5px">Evaluates each form until one produces a non-nil/non-false result, produces #f
if all expressions are 'falsy'.
The or form will stop evaluating when the first expression produces non-nil/false.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (or nil nil #t (err "and- can not happen")))<br>
(test::assert-true (or #f nil #t (err "and- can not happen")))<br>
(test::assert-true (or #f #f #t (err "and- can not happen")))<br>
(test::assert-equal #f (or nil nil nil))<br>
(test::assert-equal #f (or #f nil nil))<br>
(test::assert-equal #f (or #f nil #f))<br>
(test::assert-equal #f (or #f #f #f))<br>
(test::assert-equal "or- done" (or nil "or- done"))<br>
(test::assert-equal "or- done" (or nil nil "or- done"))<br>
(test::assert-equal 6 (or nil nil (+ 1 2 3)))<br>
(test::assert-equal 2 (or (/ 10 5) (* 5 2) (+ 1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::when" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``when``](#root::when-contents) | Type: Macro |
 ``root::when`` | ``Usage: (when provided-condition if-true)`` |

<span style="padding-left: 5px">when is a convenience function used to check a form, provided-condition,
and run some form, if-true, if provided-condition evaluates to true.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (when #t #t))<br>
(assert-false (when #t nil))<br>
(assert-false (when nil nil))<br>
<br>
</code>
</details>
<br>
### <a id="core-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Core forms](#core-contents)



| <a id="root::*collection-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*collection-src*``](#root::*collection-src*-contents) | Type: String |
 ``root::*collection-src*`` | ``Usage: (print *collection-src*)`` |

<span style="padding-left: 5px">The builtin source code for collection.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *collection-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*core-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*core-src*``](#root::*core-src*-contents) | Type: String |
 ``root::*core-src*`` | ``Usage: (print *core-src*)`` |

<span style="padding-left: 5px">The builtin source code for core.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *core-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*getopts-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*getopts-src*``](#root::*getopts-src*-contents) | Type: String |
 ``root::*getopts-src*`` | ``Usage: (print *getopts-src*)`` |

<span style="padding-left: 5px">The builtin source code for shell.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *getopts-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*iterator-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*iterator-src*``](#root::*iterator-src*-contents) | Type: String |
 ``root::*iterator-src*`` | ``Usage: (print *iterator-src*)`` |

<span style="padding-left: 5px">The builtin source code for iterator.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *iterator-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*lib-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*lib-src*``](#root::*lib-src*-contents) | Type: String |
 ``root::*lib-src*`` | ``Usage: (print *lib-src*)`` |

<span style="padding-left: 5px">The builtin source code for lib.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *lib-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*seq-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*seq-src*``](#root::*seq-src*-contents) | Type: String |
 ``root::*seq-src*`` | ``Usage: (print *seq-src*)`` |

<span style="padding-left: 5px">The builtin source code for seq.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *seq-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*shell-read-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*shell-read-src*``](#root::*shell-read-src*-contents) | Type: String |
 ``root::*shell-read-src*`` | ``Usage: (print *shell-read-src*)`` |

<span style="padding-left: 5px">The builtin source code for shell-read.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *shell-read-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*shell-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*shell-src*``](#root::*shell-src*-contents) | Type: String |
 ``root::*shell-src*`` | ``Usage: (print *shell-src*)`` |

<span style="padding-left: 5px">The builtin source code for shell.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *shell-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*slsh-std-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*slsh-std-src*``](#root::*slsh-std-src*-contents) | Type: String |
 ``root::*slsh-std-src*`` | ``Usage: (print *slsh-std-src*)`` |

<span style="padding-left: 5px">The builtin source code for slsh-std.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *slsh-std-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*slshrc-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*slshrc-src*``](#root::*slshrc-src*-contents) | Type: String |
 ``root::*slshrc-src*`` | ``Usage: (print *slshrc-src*)`` |

<span style="padding-left: 5px">The builtin source code for slshrc.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *slshrc-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*struct-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*struct-src*``](#root::*struct-src*-contents) | Type: String |
 ``root::*struct-src*`` | ``Usage: (print *struct-src*)`` |

<span style="padding-left: 5px">The builtin source code for struct.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *struct-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*test-src*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*test-src*``](#root::*test-src*-contents) | Type: String |
 ``root::*test-src*`` | ``Usage: (print *test-src*)`` |

<span style="padding-left: 5px">The builtin source code for test.lisp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(print *test-src*)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::and-let*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``and-let*``](#root::and-let*-contents) | Type: Macro |
 ``root::and-let*`` | ``Usage: (and-let* vals &rest let-body)`` |

<span style="padding-left: 5px">Takes list, vals, of form ((binding0 sexp0) (binding1 sexp1) ...) checking
if result of each sexp evaluates to false, short circuiting and returning nil
if so. If all vals bindings evaluate to true, then the let-body is evaluated
with all values of binding bound to the result of the evaluation of sexp.
Sexps can reference
bindings from previous items in the list of vals. If no let-body is supplied
the last binding in the list of vals is returned.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "charlie bravo alpha."<br>
(and-let* ((val (str "alpha."))<br>
(other-val (str "bravo " val)))<br>
(str "charlie " other-val)) "]")<br>
(test::assert-equal "alpha, bravo." (and-let* ((val (do<br>
(str "bravo.")))<br>
(other-val (do<br>
(str "alpha, " val))))))<br>
(test::assert-false (and-let* ((val (do (str "alpha, ") nil))<br>
(other-val (do (str "bravo " val))))))<br>
(test::assert-false (and-let* ((a-list (list 1 2 3 4 5 6 7 8))<br>
(evens (filter (fn (x) (= 0 (% x 2))) a-list))<br>
(doubled (map (fn (x) (* x 2)) evens))<br>
(odds (collect (filter (fn (x) (= 1 (% x 2))) doubled))))<br>
#t))<br>
<br>
</code>
</details>
<br>


| <a id="root::apply" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``apply``](#root::apply-contents) | Type: Function |
 ``root::apply`` | ``Usage: (apply function arg* list)`` |

<span style="padding-left: 5px">Call the provided function with the suplied arguments, last is a list that will be expanded.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-apply-one (apply str '("O" "NE")))<br>
(test::assert-equal "ONE" test-apply-one)<br>
(test::assert-equal 10 (apply + 1 '(2 7)))<br>
<br>
</code>
</details>
<br>


| <a id="root::back-quote" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``back-quote``](#root::back-quote-contents) | Type: SpecialForm |
 ``root::back-quote`` | ``Usage: `expression -> expression`` |

<span style="padding-left: 5px">Return expression without evaluation.
Always use the ` reader macro or expansion will not work
(i.e. (back-quote expression) will not do , expansion).
Backquote (unlike quote) allows for symbol/form evaluation using , or ,@.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal (list 1 2 3) `(1 2 3))<br>
(test::assert-equal `(1 2 3) '(1 2 3))<br>
(def test-bquote-one 1)<br>
(def test-bquote-list '(1 2 3))<br>
(test::assert-equal (list 1 2 3) `(,test-bquote-one 2 3))<br>
(test::assert-equal (list 1 2 3) `(,@test-bquote-list))<br>
<br>
</code>
</details>
<br>


| <a id="root::block" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``block``](#root::block-contents) | Type: SpecialForm |
 ``root::block`` | ``Usage: (block name form*)`` |

<span style="padding-left: 5px">Create a block with name (name is not evaluated), if no return-from encountered then
return last expression (like do).
Note: If the last expression in a block is a tail call then it can not use
return-from since the tail call will leave the block and the return-from will
not find it.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(4 5) (block xxx '(1 2) (return-from xxx '(4 5)) '(a b) '(2 3)))<br>
(test::assert-equal '(4 5) (block xxx '(1 2) (return-from nil '(4 5)) '(a b) '(2 3)))<br>
(test::assert-equal '(5 6) (block xxx '(1 2) (block yyy (return-from xxx '(5 6)) '(a b)) '(2 3)))<br>
(test::assert-equal '(5 6) (block xxx '(1 2) (block yyy ((fn (p) (return-from xxx p)) '(5 6)) '(a b)) '(2 3)))<br>
(test::assert-equal '(2 3) (block xxx '(1 2) (block yyy (return-from yyy #t) '(a b)) '(2 3)))<br>
(test::assert-equal '(5 6) (block yyy ((fn (p) (return-from yyy p)) '(5 6)) '(a b)) '(2 3))<br>
(test::assert-equal 2<br>
(block forloop<br>
(for item in '(1 2 3)<br>
(when (= 2 item)<br>
(return-from forloop item)))<br>
nil)) ; This nil keeps the for loop from being a tail call.<br>
<br>
</code>
</details>
<br>


| <a id="root::dec!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dec!``](#root::dec!-contents) | Type: Macro |
 ``root::dec!`` | ``Usage: (dec! symbol [number]) -> new value`` |

<span style="padding-left: 5px">Decrement the value in symbol by one or the optional number
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def *dec-test* 5)<br>
(test::assert-equal 4 (dec! *dec-test*))<br>
(test::assert-equal 4 *dec-test*)<br>
(test::assert-error (dec! *dec-test* "xxx"))<br>
(test::assert-equal 1 (dec! *dec-test* 3))<br>
(test::assert-equal 1 *dec-test*)<br>
(def *dec-test* "xxx")<br>
(test::assert-error (dec! *dec-test*))<br>
(let ((dec-test 5))<br>
(test::assert-equal 4 (dec! dec-test))<br>
(test::assert-equal 4 dec-test)<br>
(test::assert-equal 1 (dec! dec-test 3))<br>
(test::assert-equal 1 dec-test))<br>
<br>
</code>
</details>
<br>


| <a id="root::def" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``def``](#root::def-contents) | Type: SpecialForm |
 ``root::def`` | ``Usage: (def symbol doc_string? expression) -> expression`` |

<span style="padding-left: 5px">Adds an expression to the current namespace.  Return the expression that was defined.
Symbol is not evaluted.  Can take an option doc string (docstrings can only be
set on namespaced (global) symbols).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-do-one nil)<br>
(def test-do-two nil)<br>
(def test-do-three (do (set! test-do-one "One")(set! test-do-two "Two")"Three"))<br>
(test::assert-equal "One" test-do-one)<br>
(test::assert-equal "Two" test-do-two)<br>
(test::assert-equal "Three" test-do-three)<br>
(let ((test-do-one nil))<br>
; Add this to tthe let's scope (shadow the outer test-do-two).<br>
(test::assert-equal "Default" (def ns::test-do-four "Default"))<br>
; set the currently scoped value.<br>
(set! test-do-one "1111")<br>
(set! test-do-two "2222")<br>
(test::assert-equal "1111" test-do-one)<br>
(test::assert-equal "2222" test-do-two)<br>
(test::assert-equal "Default" test-do-four))<br>
; Original outer scope not changed.<br>
(test::assert-equal "One" test-do-one)<br>
(test::assert-equal "Default" test-do-four)<br>
;(def (sym "test-do-one") "do one")<br>
(test::assert-error (def (sym "test-do-one") "do one"))<br>
;(test::assert-equal "do one" test-do-one)<br>
(test::assert-error (def (sym->str test-do-one) "do one 2"))<br>
<br>
</code>
</details>
<br>


| <a id="root::def?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``def?``](#root::def?-contents) | Type: SpecialForm |
 ``root::def?`` | ``Usage: (def? expression) -> t\|nil`` |

<span style="padding-left: 5px">Return true if is a defined symbol (bound within the current scope). If expression
is a symbol it is not evaluted and if a list it is evaluted to produce a symbol.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-is-def #t)<br>
(def test-is-def2 'test-is-def)<br>
(test::assert-true (def? test-is-def))<br>
(test::assert-true (def? (sym "test-is-def")))<br>
(test::assert-true (def? (ref test-is-def2)))<br>
(test::assert-false (def? test-is-def-not-defined))<br>
(test::assert-false (def? (sym "test-is-def-not-defined")))<br>
(test::assert-error (def? (ref test-is-def)))<br>
<br>
</code>
</details>
<br>


| <a id="root::defmacro" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``defmacro``](#root::defmacro-contents) | Type: Macro |
 ``root::defmacro`` | ``Usage: (defmacro name doc_string? argument_list body)`` |

<span style="padding-left: 5px">Create a macro and bind it to a symbol in the current scope.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defmacro test-mac (x) (let ((y (+ (ref (ref x)) 1))) `(set! ,x ,y)))<br>
(def test-mac-x 2)<br>
(test-mac test-mac-x)<br>
(test::assert-equal 3 test-mac-x)<br>
(defmacro test-mac (x) `(set! ,x 15))<br>
(test-mac test-mac-x)<br>
(test::assert-equal 15 test-mac-x)<br>
<br>
</code>
</details>
<br>


| <a id="root::defn" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``defn``](#root::defn-contents) | Type: Macro |
 ``root::defn`` | ``Usage: (defn name &rest args)`` |

<span style="padding-left: 5px">Define a named function in the current namespace.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defn defn-test (x y) (+ x y))<br>
(test::assert-equal 5 (defn-test 2 3))<br>
(defn defn-test (x y) (set! x (* x 2))(+ x y))<br>
(test::assert-equal 7 (defn-test 2 3))<br>
(defn defn-test (x y))<br>
(test::assert-false (defn-test 2 3))<br>
(defn defn-test (x y) #t)<br>
(test::assert-true (defn-test 2 3))<br>
<br>
</code>
</details>
<br>


| <a id="root::do" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``do``](#root::do-contents) | Type: SpecialForm |
 ``root::do`` | ``Usage: (do exp0 ... expN) -> expN`` |

<span style="padding-left: 5px">Evaluatate each form and return the last.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-do-one nil)<br>
(def test-do-two nil)<br>
(def test-do-three (do (set! test-do-one "One")(set! test-do-two "Two")"Three"))<br>
(test::assert-equal "One" test-do-one)<br>
(test::assert-equal "Two" test-do-two)<br>
(test::assert-equal "Three" test-do-three)<br>
<br>
</code>
</details>
<br>


| <a id="root::doc" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``doc``](#root::doc-contents) | Type: Function |
 ``root::doc`` | ``Usage: (doc symbol)`` |

<span style="padding-left: 5px">Return the doc string for a symbol or nil if no string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(doc 'car)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::doc-raw" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``doc-raw``](#root::doc-raw-contents) | Type: Function |
 ``root::doc-raw`` | ``Usage: (doc-raw symbol)`` |

<span style="padding-left: 5px">Return the raw (unexpanded) doc string for a symbol or nil if no string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(doc-raw 'car)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::dotimes" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dotimes``](#root::dotimes-contents) | Type: Macro |
 ``root::dotimes`` | ``Usage: (dotimes times body)`` |

<span style="padding-left: 5px">Evaluate body a number of times equal to times' numerical value.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def i 0)<br>
(dotimes 11 (set! i (+ 1 i)))<br>
(assert-equal 11 i)<br>
<br>
</code>
</details>
<br>


| <a id="root::dotimes-i" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dotimes-i``](#root::dotimes-i-contents) | Type: Macro |
 ``root::dotimes-i`` | ``Usage: (dotimes-i idx-bind times body)`` |

<span style="padding-left: 5px">Evaluate body a number of times equal to times' numnrical value. Includes an
incrementing reference binding, idx-bind, accesible in body.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def i 0)<br>
(def i-tot 0)<br>
(dotimes-i idx 11 (do (set! i-tot (+ idx i-tot))(set! i (+ 1 i))))<br>
(assert-equal 11 i)<br>
(assert-equal 55 i-tot)<br>
<br>
</code>
</details>
<br>


| <a id="root::dyn" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dyn``](#root::dyn-contents) | Type: Macro |
 ``root::dyn`` | ``Usage: (dyn key value expression) -> result_of_expression`` |

<span style="padding-left: 5px">Creates a dynamic binding for key, assigns value to it and evals expression under it.
Note that if key must be a symbol and is not evaluted.
The binding is gone once the dyn form ends. This is basically a set! on the
binding in an unwind protect to reset it when done.  When used on a global will
set the first binding found and reset it when done.
Calls to dyn can be nested and previous dynamic values will
be restored as interior dyn's exit.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defn test-dyn-fn () (print "Print dyn out"))<br>
(dyn *stdout* (open "/tmp/sl-sh.dyn.test" :create :truncate) (test-dyn-fn))<br>
(test::assert-equal "Print dyn out" (read-line (open "/tmp/sl-sh.dyn.test" :read)))<br>
<br>
</code>
</details>
<br>


| <a id="root::eprint" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``eprint``](#root::eprint-contents) | Type: Function |
 ``root::eprint`` | ``Usage: (eprint arg0 ... argN) -> nil`` |

<span style="padding-left: 5px">Print the arguments (as strings) to *stderr*.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Use a file for stderr for test.<br>
(dyn *stderr* (open "/tmp/sl-sh.eprint.test" :create :truncate) (do (eprint "eprint test out")(eprint " two") (close *stderr*)))<br>
(test::assert-equal "eprint test out two" (read-line (open "/tmp/sl-sh.eprint.test" :read)))<br>
<br>
</code>
</details>
<br>


| <a id="root::eprintln" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``eprintln``](#root::eprintln-contents) | Type: Function |
 ``root::eprintln`` | ``Usage: (eprintln arg0 ... argN) -> nil`` |

<span style="padding-left: 5px">Print the arguments (as strings) to *stderr* and then a newline.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Use a file for stderr for test.<br>
(dyn *stderr* (open "/tmp/sl-sh.eprintln.test" :create :truncate) (do (eprintln "eprintln test out")(eprintln "line two") (close *stderr*)))<br>
(def topen (open "/tmp/sl-sh.eprintln.test" :read))<br>
(test::assert-equal "eprintln test out<br>
" (read-line topen))<br>
(test::assert-equal "line two<br>
" (read-line topen))<br>
<br>
</code>
</details>
<br>


| <a id="root::err" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err``](#root::err-contents) | Type: Function |
 ``root::err`` | ``Usage: (err string) -> raises an error`` |

<span style="padding-left: 5px">Raise an error with the supplied string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-err-err (get-error (err "Test Error")))<br>
(test::assert-equal :error (car test-err-err))<br>
(test::assert-equal "Test Error" (cadr test-err-err))<br>
<br>
</code>
</details>
<br>


| <a id="root::error-stack-off" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``error-stack-off``](#root::error-stack-off-contents) | Type: Lambda |
 ``root::error-stack-off`` | ``Usage: (error-stack-off)`` |

<span style="padding-left: 5px">Currently a no-op, used to turn off error stacks.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; no-op<br>
(error-stack-off)<br>
<br>
</code>
</details>
<br>


| <a id="root::error-stack-on" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``error-stack-on``](#root::error-stack-on-contents) | Type: Lambda |
 ``root::error-stack-on`` | ``Usage: (error-stack-on)`` |

<span style="padding-left: 5px">Currently a no-op, used to turn on error stacks.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; no-op<br>
(error-stack-on)<br>
<br>
</code>
</details>
<br>


| <a id="root::eval" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``eval``](#root::eval-contents) | Type: Function |
 ``root::eval`` | ``Usage: (eval expression)`` |

<span style="padding-left: 5px">Evaluate the provided expression.
If expression is a string read it to make an ast first to evaluate otherwise
evaluate the expression (note eval is a function not a special form, the
provided expression will be evaluated as part of call).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-eval-one nil)<br>
(eval (read "(set! test-eval-one \"ONE\")"))<br>
(test::assert-equal "ONE" test-eval-one)<br>
(eval '(set! test-eval-one "TWO"))<br>
(test::assert-equal "TWO" test-eval-one)<br>
<br>
</code>
</details>
<br>


| <a id="root::expand-macro" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``expand-macro``](#root::expand-macro-contents) | Type: Function |
 ``root::expand-macro`` | ``Usage: (expand-macro expression)`` |

<span style="padding-left: 5px">Expands a macro expression.  If that expansion is also a macro then expand it recursively.
Just returns the expression if not a macro.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(def xx "value") (expand-macro '(def xx "value")))<br>
(defmacro mac-test-for<br>
(bind in in_list body) (do<br>
(if (not (= in 'in)) (err "Invalid test-mac-for: (test-mac-for [v] in [iterator] (body))"))<br>
`((fn (,bind)<br>
(if (> (length ,in_list) 0)<br>
(root::loop (plist) (,in_list) (do<br>
(set! ,bind (root::first plist))<br>
(,@body)<br>
(if (> (length plist) 1) (recur (root::rest plist)))))))nil)))<br>
(test::assert-equal '(<br>
(fn<br>
(i)<br>
(if<br>
(> (length '(1 2 3)) 0)<br>
(root::loop<br>
(plist)<br>
('(1 2 3))<br>
(do<br>
(set! i (root::first plist)) nil<br>
(if<br>
(> (length plist) 1)<br>
(recur (root::rest plist))))))) nil)<br>
(expand-macro '(mac-test-for i in '(1 2 3) ())))<br>
(test::assert-equal '(1 2 3) (expand-macro '(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::expand-macro-all" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``expand-macro-all``](#root::expand-macro-all-contents) | Type: Function |
 ``root::expand-macro-all`` | ``Usage: (expand-macro-all expression)`` |

<span style="padding-left: 5px">Expands a macro expression like expand-macro but also expand any embedded macros.
Just returns the expression if not a macro.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(def xx "value") (expand-macro-all '(def xx "value")))<br>
(defmacro mac-test-loop<br>
(params bindings body)<br>
`((fn ,params ,body) ,@bindings))<br>
(defmacro mac-test-for<br>
(bind in in_list body) (do<br>
(if (not (= in 'in)) (err "Invalid test-mac-for: (test-mac-for [v] in [iterator] (body))"))<br>
`((fn (,bind)<br>
(if (> (length ,in_list) 0)<br>
(mac-test-loop (plist) (,in_list) (do<br>
(set! ,bind (root::first plist))<br>
(,@body)<br>
(if (> (length plist) 1) (recur (root::rest plist)))))))nil)))<br>
(test::assert-equal '(<br>
(fn<br>
(i)<br>
(if<br>
(> (length '(1 2 3)) 0)<br>
(<br>
(fn<br>
(plist)<br>
(do<br>
(set! i (root::first plist)) nil<br>
(if<br>
(> (length plist) 1)<br>
(recur (root::rest plist)))))<br>
'(1 2 3)))) nil)<br>
(expand-macro-all '(mac-test-for i in '(1 2 3) ())))<br>
(test::assert-equal '(1 2 3) (expand-macro-all '(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::expand-macro1" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``expand-macro1``](#root::expand-macro1-contents) | Type: Function |
 ``root::expand-macro1`` | ``Usage: (expand-macro1 expression)`` |

<span style="padding-left: 5px">Expands a macro expression.  Only expand the first macro.
Just returns the expression if not a macro.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(def xx "value") (expand-macro1 '(def xx "value")))<br>
(defmacro mac-test-for<br>
(bind in in_list body) (do<br>
(if (not (= in 'in)) (err "Invalid test-mac-for: (test-mac-for [v] in [iterator] (body))"))<br>
`((fn (,bind)<br>
(if (> (length ,in_list) 0)<br>
(root::loop (plist) (,in_list) (do<br>
(set! ,bind (root::first plist))<br>
(,@body)<br>
(if (> (length plist) 1) (recur (root::rest plist)))))))nil)))<br>
(test::assert-equal '((fn<br>
(i)<br>
(if<br>
(> (length '(1 2 3)) 0)<br>
(root::loop<br>
(plist)<br>
('(1 2 3))<br>
(do<br>
(set! i (root::first plist)) nil<br>
(if<br>
(> (length plist) 1)<br>
(recur (root::rest plist)))))))nil)<br>
(expand-macro1 '(mac-test-for i in '(1 2 3) ())))<br>
(test::assert-equal '(1 2 3) (expand-macro1 '(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::fn" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fn``](#root::fn-contents) | Type: SpecialForm |
 ``root::fn`` | ``Usage: (fn (param*) expr*) -> exprN`` |

<span style="padding-left: 5px">Create a function (lambda).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-fn1 nil)<br>
(def test-fn2 nil)<br>
(def test-fn3 nil)<br>
(def test-fn-empty ((fn ())))<br>
(test::assert-false test-fn-empty)<br>
((fn () (set! test-fn1 1)))<br>
(test::assert-equal 1 test-fn1)<br>
((fn () (set! test-fn1 10)(set! test-fn2 2)))<br>
(test::assert-equal 10 test-fn1)<br>
(test::assert-equal 2 test-fn2)<br>
((fn () (set! test-fn1 11)(set! test-fn2 20)(set! test-fn3 3)))<br>
(test::assert-equal 11 test-fn1)<br>
(test::assert-equal 20 test-fn2)<br>
(test::assert-equal 3 test-fn3)<br>
((fn (x y z) (set! test-fn1 x)(set! test-fn2 y)(set! test-fn3 z)) 12 21 30)<br>
(test::assert-equal 12 test-fn1)<br>
(test::assert-equal 21 test-fn2)<br>
(test::assert-equal 30 test-fn3)<br>
(test::assert-equal 63 ((fn (x y z) (set! test-fn1 x)(set! test-fn2 y)(set! test-fn3 z)(+ x y z)) 12 21 30))<br>
<br>
</code>
</details>
<br>


| <a id="root::format" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``format``](#root::format-contents) | Type: Function |
 ``root::format`` | ``Usage: (format arg0 ... argN) -> string`` |

<span style="padding-left: 5px">Build a formatted string from arguments.
Arguments will be turned into strings.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "stringsome" (format "string" "some"))<br>
(test::assert-equal "string" (format "string" ""))<br>
(test::assert-equal "string 50" (format "string" " " 50))<br>
(test::assert-equal "string 50 100.5" (format "string" " " 50 " " 100.5))<br>
<br>
</code>
</details>
<br>


| <a id="root::gensym" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``gensym``](#root::gensym-contents) | Type: Function |
 ``root::gensym`` | ``Usage: (gensym) -> symbol`` |

<span style="padding-left: 5px">Generate a unique symbol.
Gensym uses a prefix of gs@@ followed by an incrementing counter.
It is useful to generate unique symbol names when writing macros (for instance).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-gensym-one (gensym))<br>
(def test-gensym-two (gensym))<br>
(def test-gensym-three (gensym))<br>
(test::assert-true (str-starts-with "gs@@" (sym->str test-gensym-one)))<br>
(test::assert-true (str-starts-with "gs@@" (sym->str test-gensym-two)))<br>
(test::assert-true (str-starts-with "gs@@" (sym->str test-gensym-three)))<br>
(test::assert-true (symbol? (gensym)))<br>
(test::assert-true (symbol? test-gensym-one))<br>
(test::assert-true (symbol? test-gensym-two))<br>
(test::assert-true (symbol? test-gensym-three))<br>
<br>
</code>
</details>
<br>


| <a id="root::get-error" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-error``](#root::get-error-contents) | Type: Function |
 ``root::get-error`` | ``Usage: (get-error exp0 ... expN) -> pair`` |

<span style="padding-left: 5px">Evaluate each form (like do) but on error return (:error msg backtrace) instead of aborting.
On success return (:ok . expN-result).
If there is no error will return the value of the last expression as the cdr of
the pair.  Always returns a pair with the first value either being :ok or :error.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def get-error-t1 (get-error (err "Some Error")))<br>
(test::assert-equal :error (car get-error-t1))<br>
(test::assert-equal "Some Error" (cadr get-error-t1))<br>
(test::assert-true (vec? (caddr get-error-t1)))<br>
(test::assert-equal '(:ok . "Some String") (get-error "Some String"))<br>
(test::assert-equal '(:ok . "Some Other String") (get-error (def test-get-error "Some ") (str test-get-error "Other String")))<br>
<br>
</code>
</details>
<br>


| <a id="root::identity" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``identity``](#root::identity-contents) | Type: Lambda |
 ``root::identity`` | ``Usage: (identity x)`` |

<span style="padding-left: 5px">Identity function.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 0 (identity 0))<br>
<br>
</code>
</details>
<br>


| <a id="root::inc!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``inc!``](#root::inc!-contents) | Type: Macro |
 ``root::inc!`` | ``Usage: (inc! symbol [number]) -> new value`` |

<span style="padding-left: 5px">Increment the value in symbol by one or the optional number
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def *inc-test* 1)<br>
(test::assert-equal 2 (inc! *inc-test*))<br>
(test::assert-equal 2 *inc-test*)<br>
(test::assert-equal 5 (inc! *inc-test* 3))<br>
(test::assert-error (inc! *inc-test* "xxx"))<br>
(test::assert-equal 5 *inc-test*)<br>
(def *inc-test* "xxx")<br>
(test::assert-error (inc! *inc-test*))<br>
(let ((inc-test 1))<br>
(test::assert-equal 2 (inc! inc-test))<br>
(test::assert-equal 2 inc-test)<br>
(test::assert-equal 5 (inc! inc-test 3))<br>
(test::assert-equal 5 inc-test))<br>
<br>
</code>
</details>
<br>


| <a id="root::intern-stats" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``intern-stats``](#root::intern-stats-contents) | Type: SpecialForm |
 ``root::intern-stats`` | ``Usage: (intern-stats)`` |

<span style="padding-left: 5px">Prints the stats for interned symbols.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(intern-stats)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::len0?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``len0?``](#root::len0?-contents) | Type: Macro |
 ``root::len0?`` | ``Usage: (len0? thing)`` |

<span style="padding-left: 5px">Is the length of thing 0?
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (len0? nil))<br>
(assert-true (len0? '()))<br>
(assert-true (len0? (vec)))<br>
(assert-true (len0? (str)))<br>
(assert-true (len0? ""))<br>
(assert-false (len0? '(1)))<br>
(assert-false (len0? (vec 1 2)))<br>
(assert-false (len0? "string"))<br>
<br>
</code>
</details>
<br>


| <a id="root::len>0?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``len>0?``](#root::len>0?-contents) | Type: Macro |
 ``root::len>0?`` | ``Usage: (len>0? thing)`` |

<span style="padding-left: 5px">Is the length of thing greater than 0?
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-false (len>0? nil))<br>
(assert-false (len>0? '()))<br>
(assert-false (len>0? (vec)))<br>
(assert-false (len>0? (str)))<br>
(assert-false (len>0? ""))<br>
(assert-true (len>0? '(1)))<br>
(assert-true (len>0? (vec 1 2)))<br>
(assert-true (len>0? "string"))<br>
<br>
</code>
</details>
<br>


| <a id="root::length" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``length``](#root::length-contents) | Type: Function |
 ``root::length`` | ``Usage: (length expression) -> int`` |

<span style="padding-left: 5px">Return length of suplied expression.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 0 (length nil))<br>
(test::assert-equal 5 (length "12345"))<br>
; Note the unicode symbol is only one char even though it is more then one byte.<br>
(test::assert-equal 6 (length "12345Σ"))<br>
(test::assert-equal 3 (length '(1 2 3)))<br>
(test::assert-equal 3 (length '#(1 2 3)))<br>
(test::assert-equal 3 (length (list 1 2 3)))<br>
(test::assert-equal 3 (length (vec 1 2 3)))<br>
(test::assert-error (length 100))<br>
(test::assert-error (length 100.0))<br>
(test::assert-error (length #\x))<br>
<br>
</code>
</details>
<br>


| <a id="root::let" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``let``](#root::let-contents) | Type: Macro |
 ``root::let`` | ``Usage: (let vals &rest let-body)`` |

<span style="padding-left: 5px">Takes list, vals, of form ((binding0 sexp0) (binding1 sexp1) ...) and evaluates
let-body with all values of binding bound to the result of the evaluation of
sexp.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-do-one "One1")<br>
(def test-do-two "Two1")<br>
(def test-do-three (let ((test-do-one "One")) (set! test-do-two "Two")(test::assert-equal "One" test-do-one)"Three"))<br>
(test::assert-equal "One1" test-do-one)<br>
(test::assert-equal "Two" test-do-two)<br>
(test::assert-equal "Three" test-do-three)<br>
((fn (idx) (let ((v2 (+ idx 2))(v3 (+ idx 3)))<br>
(test::assert-equal (+ idx 2) v2)<br>
(test::assert-equal (+ idx 3) v3)<br>
(if (< idx 5) (recur (+ idx 1)))))0)<br>
((fn (idx) (let ((v2 (+ idx 2))(v3 (+ idx 3)))<br>
(test::assert-equal (+ idx 2) v2)<br>
(test::assert-equal (+ idx 3) v3)<br>
(if (< idx 5) (this-fn (+ idx 1)))))0)<br>
<br>
</code>
</details>
<br>


| <a id="root::let*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``let*``](#root::let*-contents) | Type: Macro |
 ``root::let*`` | ``Usage: (let* vals &rest let-body)`` |

<span style="padding-left: 5px">Takes list, vals, of form ((binding0 sexp0) (binding1 sexp1) ...) and evaluates
let-body with all values of binding bound to the result of the evaluation of
sexp. Differs from let in that sexps can reference bindings from previous items
in the list of vals.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(let* ((add-one (fn (x) (+ 1 x)))<br>
(double-add (fn (x) (add-one (add-one x)))))<br>
(test::assert-equal 4 (add-one 3))<br>
(test::assert-equal 6 (double-add 4)))<br>
<br>
</code>
</details>
<br>


| <a id="root::loop" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``loop``](#root::loop-contents) | Type: Macro |
 ``root::loop`` | ``Usage: (loop params bindings &rest body)`` |

<span style="padding-left: 5px">Binds bindings to parameters in body. Use recur with desired bindings for
subsequent iteration.
Within the loop the lambda 'break' will end the loop, break can take an option
argument that is what the loop produces (nil if no argument).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tot 0)<br>
(loop (idx) (3) (do<br>
(set! tot (+ tot 1))<br>
(if (> idx 1) (recur (- idx 1)))))<br>
(assert-equal 3 tot)<br>
(def tot 0)<br>
(loop (idx) (0)<br>
(set! tot (+ tot 1))<br>
(if (= idx 2) (break))<br>
(recur (+ idx 1)))<br>
(assert-equal 3 tot)<br>
(assert-equal 11 (loop (idx) (0)<br>
(if (= idx 2) (break 11))<br>
(recur (+ idx 1))))<br>
(assert-false (loop (idx) (0)<br>
(if (= idx 2) (break))<br>
(recur (+ idx 1))))<br>
(assert-error (loop (idx) (0)<br>
(if (= idx 2) (break 1 3))<br>
(recur (+ idx 1))))<br>
<br>
</code>
</details>
<br>


| <a id="root::macro" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``macro``](#root::macro-contents) | Type: SpecialForm |
 ``root::macro`` | ``Usage: (macro (args) `(apply + ,@args))`` |

<span style="padding-left: 5px">Define an anonymous macro.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-macro1 nil)<br>
(def test-macro2 nil)<br>
(def test-macro-empty (macro ()))<br>
(test::assert-false (test-macro-empty))<br>
(def test-mac nil)<br>
(def mac-var 2)<br>
(let ((mac-var 3))<br>
(set! test-mac (macro (x) (set! test-macro2 100)(test::assert-equal 3 mac-var)`(* ,mac-var ,x))))<br>
(set! test-macro1 (test-mac 10))<br>
(test::assert-equal 30 test-macro1)<br>
(test::assert-equal 100 test-macro2)<br>
<br>
</code>
</details>
<br>


| <a id="root::maybe-docstring?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``maybe-docstring?``](#root::maybe-docstring?-contents) | Type: Lambda |
 ``root::maybe-docstring?`` | ``Usage: (maybe-docstring? form)`` |

<span style="padding-left: 5px">True if form might be a docstring, nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (maybe-docstring? "string"))<br>
(test::assert-true (maybe-docstring? '(str 1 2 3)))<br>
(test::assert-false (maybe-docstring? '(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::meta-add-tags" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-add-tags``](#root::meta-add-tags-contents) | Type: Function |
 ``root::meta-add-tags`` | ``Usage: (meta-add-tags expression tag*)`` |

<span style="padding-left: 5px">Adds multiple meta tags to an expression (see meta-add-tag).  It will work with
symbols or vectors or lists of symbols (or any combination).
This is intended for helping with structs and interfaces in lisp, you probably
do not want to use it.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def meta-add-tags-var '(1 2 3))<br>
(meta-add-tags meta-add-tags-var :tag1)<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag1))<br>
(test::assert-false (meta-tag? meta-add-tags-var :tag2))<br>
(meta-add-tags meta-add-tags-var :tag2 '(:tag3 :tag4) '#(:tag5 :tag6) :tag7)<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag2))<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag3))<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag4))<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag5))<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag6))<br>
(test::assert-true (meta-tag? meta-add-tags-var :tag7))<br>
<br>
</code>
</details>
<br>


| <a id="root::meta-column-no" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-column-no``](#root::meta-column-no-contents) | Type: SpecialForm |
 ``root::meta-column-no`` | ``Usage: (meta-column-no)`` |

<span style="padding-left: 5px">Column number from the file this came from.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(meta-column-no)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::meta-file-name" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-file-name``](#root::meta-file-name-contents) | Type: SpecialForm |
 ``root::meta-file-name`` | ``Usage: (meta-file-name)`` |

<span style="padding-left: 5px">File name of the file this came from.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(meta-file-name)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::meta-line-no" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-line-no``](#root::meta-line-no-contents) | Type: SpecialForm |
 ``root::meta-line-no`` | ``Usage: (meta-line-no)`` |

<span style="padding-left: 5px">Line number from the file this came from.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(meta-line-no)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::meta-tag?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meta-tag?``](#root::meta-tag?-contents) | Type: Function |
 ``root::meta-tag?`` | ``Usage: (meta-tag? expression tag)`` |

<span style="padding-left: 5px">True if expression has the meta tag 'tag' set.  This is intended for helping
with structs and interfaces in lisp, you probably do not want to use it.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def meta-add-tag-var '(1 2 3))<br>
(meta-add-tags meta-add-tag-var :tag1)<br>
(test::assert-true (meta-tag? meta-add-tag-var :tag1))<br>
(test::assert-false (meta-tag? meta-add-tag-var :tag2))<br>
<br>
</code>
</details>
<br>


| <a id="root::nsubstitute!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``nsubstitute!``](#root::nsubstitute!-contents) | Type: Lambda |
 ``root::nsubstitute!`` | ``Usage: (nsubstitute! new-item old-item lst &rest mods)`` |

<span style="padding-left: 5px">Replaces all instances of old-item in lst with new-item. If last argument
passed in is keyword :first only the first instance of old-item will be
replaced.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(let ((lst (list 1 2 3 4 5)))<br>
(test::assert-equal (list 1 2 10 4 5) (nsubstitute! 10 3 lst))<br>
(test::assert-equal (list 1 2 10 4 5) lst))<br>
<br>
</code>
</details>
<br>


| <a id="root::occurs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``occurs``](#root::occurs-contents) | Type: Lambda |
 ``root::occurs`` | ``Usage: (occurs (list 1 2 ...) 7) (occurs (list 1 2 ...) 0 (fn (x) (% x 2)))`` |

<span style="padding-left: 5px">Counts instances of item in sequence. Optional third argument is a function
that can be applied to the specific element in the list before equality
is tested with item.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 7 (occurs (list 1 3 5 2 4 10 2 4 88 2 1) 0 (fn (x) (% x 2))))<br>
(test::assert-equal 3 (occurs (list 1 3 5 2 4 10 2 4 88 2 1) 2))<br>
(test::assert-equal 0 (occurs (list 1 3 5 2 4 10 2 4 88 2 1) 42))<br>
(test::assert-equal 2 (occurs (list 1 3 5 2 4 10 2 4 88 2 1) 8 (fn (x) (* x 2))))<br>
<br>
</code>
</details>
<br>


| <a id="root::print" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``print``](#root::print-contents) | Type: Function |
 ``root::print`` | ``Usage: (print arg0 ... argN) -> nil`` |

<span style="padding-left: 5px">Print the arguments (as strings) to *stdout*.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Use a file for stdout for test.<br>
(dyn *stdout* (open "/tmp/sl-sh.print.test" :create :truncate) (do (print "Print test out")(print " two") (close *stdout*)))<br>
(test::assert-equal "Print test out two" (read-line (open "/tmp/sl-sh.print.test" :read)))<br>
<br>
</code>
</details>
<br>


| <a id="root::print-error" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``print-error``](#root::print-error-contents) | Type: Lambda |
 ``root::print-error`` | ``Usage: (print-error error)`` |

<span style="padding-left: 5px">Prints out an error with a backtrace.  Used with the return of get-error
when it produces an error
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(let<br>
((print-error-test (get-error (err "Oops!")))<br>
(file-name "$(temp-dir)/print-error.test")<br>
(topen))<br>
(out> file-name (print-error print-error-test))<br>
(set! topen (open file-name :read))<br>
(test::assert-true (> (length (read-line topen)) 5))<br>
(close topen))<br>
<br>
</code>
</details>
<br>


| <a id="root::println" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``println``](#root::println-contents) | Type: Function |
 ``root::println`` | ``Usage: (println arg0 ... argN) -> nil`` |

<span style="padding-left: 5px">Print the arguments (as strings) to *stdout* and then a newline.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Use a file for stdout for test.<br>
(dyn *stdout* (open "/tmp/sl-sh.println.test" :create :truncate) (do (println "Println test out")(println "line two") (close *stdout*)))<br>
(def topen (open "/tmp/sl-sh.println.test" :read))<br>
(test::assert-equal "Println test out<br>
" (read-line topen))<br>
(test::assert-equal "line two<br>
" (read-line topen))<br>
<br>
</code>
</details>
<br>


| <a id="root::quote" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``quote``](#root::quote-contents) | Type: SpecialForm |
 ``root::quote`` | ``Usage: 'expression -> expression`` |

<span style="padding-left: 5px">Return expression without evaluation.
The reader macro 'expression will expand to (quote expression).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal (list 1 2 3) (quote (1 2 3)))<br>
(test::assert-equal (list 1 2 3) '(1 2 3))<br>
(test::assert-equal '(1 2 3) (quote (1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::reader-macro-dot" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reader-macro-dot``](#root::reader-macro-dot-contents) | Type: Lambda |
 ``root::reader-macro-dot`` | ``Usage: (reader-macro-dot stream ch)`` |

<span style="padding-left: 5px">Reader macro for #.(...).  Do not call directly.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def dot-test (read "(1 2 #.(* 3 10) #.(str "o" "ne"))))<br>
(test::assert-equal '(1 2 30 "one"))<br>
<br>
</code>
</details>
<br>


| <a id="root::recur" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``recur``](#root::recur-contents) | Type: Function |
 ``root::recur`` | ``Usage: (recur &rest)`` |

<span style="padding-left: 5px">Recursively call the enclosing function with the given parameters.  Recur uses
tail call optimization and must be in the tail position or it is an error.  For
a named function it would be equivalent to a normal recursive call in a tail
position but it requires a tail position and does not need a name (a normal
recursive call would work in a non-tail position but could blow the stack if
it is to deep- unlike a recur or tail position recursive call).
NOTE: potential footgun, the let macro expands to a lambda (fn) and a recur used
inside the let would bind with the let not the enclosing lambda (this would
apply to any macro that also expands to a lamda- this is by design with the
loop macro but would be unexpected with let).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tot 0)<br>
(loop (idx) (3) (do<br>
(set! tot (+ tot 1))<br>
(if (> idx 1) (recur (- idx 1)))))<br>
(assert-equal 3 tot)<br>
(set! tot 0)<br>
((fn (idx) (do<br>
(set! tot (+ tot 1))<br>
(if (> idx 1) (recur (- idx 1)))))5)<br>
(assert-equal 5 tot)<br>
<br>
</code>
</details>
<br>


| <a id="root::ref" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ref``](#root::ref-contents) | Type: SpecialForm |
 ``root::ref`` | ``Usage: (ref symbol) -> expression`` |

<span style="padding-left: 5px">Return the expression that is referenced by symbol.
Symbol is only evaluated if a list (that produces a symbol) and must be bound
in the current scope or an error is raised.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-is-def #t)<br>
(def test-is-def2 'test-is-def)<br>
(test::assert-true (ref test-is-def))<br>
(set! test-is-def '(1 2 3))<br>
(test::assert-equal '(1 2 3) (ref test-is-def))<br>
(test::assert-error (ref test-is-def-no-exist))<br>
(test::assert-error (ref (ref test-is-def)))<br>
(test::assert-equal '(1 2 3) (ref (ref test-is-def2)))<br>
<br>
</code>
</details>
<br>


| <a id="root::return-from" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``return-from``](#root::return-from-contents) | Type: SpecialForm |
 ``root::return-from`` | ``Usage: (return-from name expression?)`` |

<span style="padding-left: 5px">Causes enclosing block with name (name is not evaluated) to evaluate to expression.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(4 5) (block xxx '(1 2) (return-from xxx '(4 5)) '(a b) '(2 3)))<br>
(test::assert-equal '(4 5) (block xxx '(1 2) (return-from nil '(4 5)) '(a b) '(2 3)))<br>
(test::assert-equal '(5 6) (block xxx '(1 2) (block yyy (return-from xxx '(5 6)) '(a b)) '(2 3)))<br>
(test::assert-equal '(5 6) (block xxx '(1 2) (block yyy ((fn (p) (return-from xxx p)) '(5 6)) '(a b)) '(2 3)))<br>
(test::assert-equal '(2 3) (block xxx '(1 2) (block yyy (return-from yyy #t) '(a b)) '(2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::set!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``set!``](#root::set!-contents) | Type: SpecialForm |
 ``root::set!`` | ``Usage: (set! symbol expression) -> expression`` |

<span style="padding-left: 5px">Sets an existing expression in the current scope(s).  Return the expression that was set.
Symbol is not evaluted.
Set will set the first binding it finds starting in the current scope and then
trying enclosing scopes until exhausted.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-do-one nil)<br>
(def test-do-two nil)<br>
(def test-do-three (do (set! test-do-one "One")(set! test-do-two "Two")"Three"))<br>
(test::assert-equal "One" test-do-one)<br>
(test::assert-equal "Two" test-do-two)<br>
(test::assert-equal "Three" test-do-three)<br>
(let ((test-do-one nil))<br>
; set the currently scoped value.<br>
(test::assert-equal "1111" (set! test-do-one "1111"))<br>
(test::assert-equal "1111" test-do-one))<br>
; Original outer scope not changed.<br>
(test::assert-equal "One" test-do-one)<br>
;(set! (sym "test-do-one") "do one")<br>
;(test::assert-equal "do one" test-do-one)<br>
(test::assert-error (set! (sym->str test-do-one) "do one 2"))<br>
<br>
</code>
</details>
<br>


| <a id="root::substitute" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``substitute``](#root::substitute-contents) | Type: Macro |
 ``root::substitute`` | ``Usage: (substitute new-item old-item lst &rest mods)`` |

<span style="padding-left: 5px">Replaces all instances of old-item in copy of lst with new-item.  If last
argument passed in is keyword :first only the first instance of old-item will be
replaced.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(let ((lst (list 1 2 3 4 5))<br>
(lst2 (list 1 2 3 3 3 4 5)))<br>
(test::assert-equal (list 1 2 10 4 5) (substitute 10 3 lst))<br>
(test::assert-equal lst lst)<br>
(test::assert-equal (list 1 2 4 4 4 4 5) (substitute 4 3 lst2))<br>
(test::assert-equal (list 1 2 4 3 3 4 5) (substitute 4 3 lst2 :first)))<br>
<br>
</code>
</details>
<br>


| <a id="root::undef" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``undef``](#root::undef-contents) | Type: SpecialForm |
 ``root::undef`` | ``Usage: (undef symbol) -> expression`` |

<span style="padding-left: 5px">Remove a symbol from the current namespace (if it exists).  Returns the expression
that was removed.  It is an error if symbol is not defined in the current namespace.
Symbol is not evaluted.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-undef nil)<br>
(test::assert-true (def? test-undef))<br>
(undef test-undef)<br>
(test::assert-false (def? test-undef))<br>
(def test-undef "undef")<br>
(test::assert-equal "undef" (undef test-undef))<br>
(test::assert-false (def? test-undef))<br>
(test::assert-equal "undef: symbol test-undef not defined in current scope (can only undef symbols in current scope)." (cadr (get-error (undef test-undef))))<br>
<br>
</code>
</details>
<br>


| <a id="root::unwind-protect" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unwind-protect``](#root::unwind-protect-contents) | Type: Function |
 ``root::unwind-protect`` | ``Usage: (unwind-protect protected cleanup*) -> [protected result]`` |

<span style="padding-left: 5px">After evaluation first form, make sure the following cleanup forms run (returns first form's result).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-unwind-one nil)<br>
(def test-unwind-err (get-error<br>
(unwind-protect (err "Some protected error") (set! test-unwind-one "got it"))))<br>
(test::assert-equal :error (car test-unwind-err))<br>
(test::assert-equal "Some protected error" (cadr test-unwind-err))<br>
(test::assert-equal "got it" test-unwind-one)<br>
(def test-unwind-one nil)<br>
(def test-unwind-two nil)<br>
(def test-unwind-three nil)<br>
(def test-unwind-four nil)<br>
(def test-unwind-err (get-error<br>
(unwind-protect<br>
(do (set! test-unwind-one "set one")(err "Some protected error two")(set! test-unwind-two "set two"))<br>
(set! test-unwind-three "set three")(set! test-unwind-four "set four"))))<br>
(test::assert-equal :error (car test-unwind-err))<br>
(test::assert-equal "Some protected error two" (cadr test-unwind-err))<br>
(test::assert-equal "set one" test-unwind-one)<br>
(test::assert-equal nil test-unwind-two)<br>
(test::assert-equal "set three" test-unwind-three)<br>
(test::assert-equal "set four" test-unwind-four)<br>
<br>
</code>
</details>
<br>


| <a id="root::values" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values``](#root::values-contents) | Type: Function |
 ``root::values`` | ``Usage: (values expression*)`` |

<span style="padding-left: 5px">Produces a multi values object.  Useful for returning more then one value from
a function when most of time you only care about the first (primary) item.  When
evaluting a muti values object it will evaluate as if it the first item only.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (values? (values 1 "str" 5.5)))<br>
(test::assert-equal 1 (values-nth 0 (values 1 "str" 5.5)))<br>
(test::assert-equal "str" (values-nth 1 (values 1 "str" 5.5)))<br>
(test::assert-equal 5.5 (values-nth 2 (values 1 "str" 5.5)))<br>
<br>
</code>
</details>
<br>


| <a id="root::values-length" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values-length``](#root::values-length-contents) | Type: Function |
 ``root::values-length`` | ``Usage: (values-length expression)`` |

<span style="padding-left: 5px">If expression is a values object then return it's length (number of values).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 3 (values-length (values 1 "str" 5.5)))<br>
(test::assert-equal 2 (values-length (values 1 "str")))<br>
(test::assert-equal 1 (values-length (values "str")))<br>
(test::assert-equal 0 (values-length (values)))<br>
(test::assert-equal "str" (values-nth 1 (values 1 "str" 5.5)))<br>
(test::assert-equal 5.5 (values-nth 2 (values 1 "str" 5.5)))<br>
(def test-vals-len (values 1 "str" 5.5))<br>
(test::assert-equal 3 (values-length test-vals-len))<br>
<br>
</code>
</details>
<br>


| <a id="root::values-nth" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values-nth``](#root::values-nth-contents) | Type: Function |
 ``root::values-nth`` | ``Usage: (values-nth idx expression)`` |

<span style="padding-left: 5px">If expression is a values object then return the item at index idx.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 1 (values-nth 0 (values 1 "str" 5.5)))<br>
(test::assert-equal "str" (values-nth 1 (values 1 "str" 5.5)))<br>
(test::assert-equal 5.5 (values-nth 2 (values 1 "str" 5.5)))<br>
(def test-vals-nth (values 1 "str" 5.5))<br>
(test::assert-equal 1 (values-nth 0 test-vals-nth))<br>
(test::assert-equal "str" (values-nth 1 test-vals-nth))<br>
(test::assert-equal 5.5 (values-nth 2 test-vals-nth))<br>
<br>
</code>
</details>
<br>


| <a id="root::var" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``var``](#root::var-contents) | Type: SpecialForm |
 ``root::var`` | ``Usage: (var symbol expression) -> expression`` |

<span style="padding-left: 5px">NOTE: var is deprecated, use let or let* to create local bindings.
Adds an expression to the current lexical scope.  Return the expression that was defined.
This will not add to a namespace (use def for that), use it within functions or
let forms to create local bindings.
Symbol is not evaluted.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(let (())<br>
(var test-do-one nil)<br>
(var test-do-two nil)<br>
(var test-do-three (do (set! test-do-one "One")(set! test-do-two "Two")"Three"))<br>
(test::assert-equal "One" test-do-one)<br>
(test::assert-equal "Two" test-do-two)<br>
(test::assert-equal "Three" test-do-three)<br>
(let ((test-do-one nil))<br>
; Add this to the let's scope (shadow the outer test-do-two).<br>
(test::assert-equal "Default" (var test-do-two "Default"))<br>
; set the currently scoped value.<br>
(set! test-do-one "1111")<br>
(set! test-do-two "2222")<br>
(test::assert-equal "1111" test-do-one)<br>
(test::assert-equal "2222" test-do-two))<br>
; Original outer scope not changed.<br>
(test::assert-equal "One" test-do-one)<br>
(test::assert-equal "Two" test-do-two))<br>
<br>
</code>
</details>
<br>
### <a id="file-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[File forms](#file-contents)
 Options to open, one or more of these can be added to open after the filename.
A file can only be opened for reading or writing (read is default).

Option | Description
-------|-----------
:read | Open file for reading, this is the default.
:write | Open file for writing.
:append | Open file for writing and append new data to end.
:truncate | Open file for write and delete all existing data.
:create | Create the file if it does not exist and open for writing.
:create-new | Create if does not exist, error if it does and open for writing.
:on-error-nil | If open has an error then return nil instead of producing an error.

Notes on closing.  Files will close when they go out of scope.  Using close will
cause a reference to a file to be marked close (removes that reference).  If
there are more then one references to a file it will not actually close until
all are released.  Close will also flush the file even if it is not the final
reference.  If a reference to a file is captured in a closure that can also keep
it open (closures currently capture the entire scope not just used symbols).


| <a id="root::cd" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cd``](#root::cd-contents) | Type: Function |
 ``root::cd`` | ``Usage: (cd dir-to-change-to)`` |

<span style="padding-left: 5px">Change directory.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(syscall 'mkdir "/tmp/tst-fs-cd")<br>
(syscall 'touch "/tmp/tst-fs-cd/fs-cd-marker")<br>
(test::assert-false (fs-exists? "fs-cd-marker"))<br>
(pushd "/tmp/tst-fs-cd")<br>
(root::cd "/tmp")<br>
(root::cd "/tmp/tst-fs-cd")<br>
(test::assert-true (fs-exists? "fs-cd-marker"))<br>
(syscall 'rm "/tmp/tst-fs-cd/fs-cd-marker")<br>
(popd)<br>
(syscall 'rmdir "/tmp/tst-fs-cd")<br>
<br>
</code>
</details>
<br>


| <a id="root::close" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``close``](#root::close-contents) | Type: Function |
 ``root::close`` | ``Usage: (close file)`` |

<span style="padding-left: 5px">Close a file.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line tst-file "Test Line Two")<br>
(close tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal "Test Line Two<br>
" (read-line tst-file))<br>
(close tst-file)<br>
<br>
</code>
</details>
<br>


| <a id="root::flush" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``flush``](#root::flush-contents) | Type: Function |
 ``root::flush`` | ``Usage: (flush file)`` |

<span style="padding-left: 5px">Flush a file.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line tst-file "Test Line Three")<br>
(flush tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal "Test Line Three<br>
" (read-line tst-file))<br>
(close tst-file)<br>
<br>
</code>
</details>
<br>


| <a id="root::fs-dir?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fs-dir?``](#root::fs-dir?-contents) | Type: Function |
 ``root::fs-dir?`` | ``Usage: (fs-dir? path-to-test)`` |

<span style="padding-left: 5px">Is the given path a directory?
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
$(mkdir /tmp/tst-fs-dir)<br>
$(touch /tmp/tst-fs-dir/fs-dir-file)<br>
(test::assert-false (fs-dir? "/tmp/tst-fs-dir/fs-dir-file"))<br>
(test::assert-true (fs-dir? "/tmp/tst-fs-dir"))<br>
(test::assert-false (fs-dir? "/tmp/tst-fs-dir/fs-dir-nope"))<br>
$(rm /tmp/tst-fs-dir/fs-dir-file)<br>
$(rmdir /tmp/tst-fs-dir)<br>
<br>
</code>
</details>
<br>


| <a id="root::fs-exists?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fs-exists?``](#root::fs-exists?-contents) | Type: Function |
 ``root::fs-exists?`` | ``Usage: (fs-exists? path-to-test)`` |

<span style="padding-left: 5px">Does the given path exist?
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
$(mkdir /tmp/tst-fs-exists)<br>
$(touch /tmp/tst-fs-exists/fs-exists)<br>
(test::assert-true (fs-exists? "/tmp/tst-fs-exists/fs-exists"))<br>
(test::assert-true (fs-exists? "/tmp/tst-fs-exists"))<br>
(test::assert-false (fs-exists? "/tmp/tst-fs-exists/fs-exists-nope"))<br>
$(rm /tmp/tst-fs-exists/fs-exists)<br>
$(rmdir /tmp/tst-fs-exists)<br>
<br>
</code>
</details>
<br>


| <a id="root::fs-file?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fs-file?``](#root::fs-file?-contents) | Type: Function |
 ``root::fs-file?`` | ``Usage: (fs-file? path-to-test)`` |

<span style="padding-left: 5px">Is the given path a file?
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
$(mkdir /tmp/tst-fs-file)<br>
$(touch "/tmp/tst-fs-file/fs-file")<br>
(test::assert-true (fs-file? "/tmp/tst-fs-file/fs-file"))<br>
(test::assert-false (fs-file? "/tmp/tst-fs-file"))<br>
(test::assert-false (fs-file? "/tmp/tst-fs-file/fs-file-nope"))<br>
$(rm "/tmp/tst-fs-file/fs-file")<br>
$(rmdir /tmp/tst-fs-file)<br>
<br>
</code>
</details>
<br>


| <a id="root::glob" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``glob``](#root::glob-contents) | Type: Function |
 ``root::glob`` | ``Usage: (glob /path/with/*)`` |

<span style="padding-left: 5px">Takes a list/varargs of globs and return the list of them expanded.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(syscall 'mkdir "/tmp/tst-fs-glob")<br>
(syscall 'touch "/tmp/tst-fs-glob/g1")<br>
(syscall 'touch "/tmp/tst-fs-glob/g2")<br>
(syscall 'touch "/tmp/tst-fs-glob/g3")<br>
(test::assert-equal '("/tmp/tst-fs-glob/g1" "/tmp/tst-fs-glob/g2" "/tmp/tst-fs-glob/g3") (glob "/tmp/tst-fs-glob/*"))<br>
(syscall 'rm "/tmp/tst-fs-glob/g1")<br>
(syscall 'rm "/tmp/tst-fs-glob/g2")<br>
(syscall 'rm "/tmp/tst-fs-glob/g3")<br>
(syscall 'rmdir "/tmp/tst-fs-glob")<br>
<br>
</code>
</details>
<br>


| <a id="root::open" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``open``](#root::open-contents) | Type: Function |
 ``root::open`` | ``Usage: (open filename option*)`` |

<span style="padding-left: 5px">Open a file.
Options are:
:read
:write
:append
:truncate
:create
:create-new
:on-error-nil
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-open-f (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line test-open-f "Test Line One")<br>
(close test-open-f)<br>
(test::assert-equal "Test Line One<br>
" (read-line (open "/tmp/slsh-tst-open.txt")))<br>
<br>
</code>
</details>
<br>


| <a id="root::read" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``read``](#root::read-contents) | Type: Function |
 ``root::read`` | ``Usage: (read [file\|string]? end-exp?) -> expression`` |

<span style="padding-left: 5px">Read a file or string and return the next object (symbol, string, list, etc).
Raises an error if the file or string has been read unless end-exp is provided
then returns that on the end condition.
If no parameters are provided then read stdin.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line tst-file "(1 2 3)(x y z)")<br>
;(write-string tst-file "Test Line Read Line Two")<br>
(flush tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal '(1 2 3) (read tst-file))<br>
(test::assert-equal '(x y z) (read tst-file))<br>
(test::assert-error (read test-file))<br>
(close tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal '(1 2 3) (read tst-file :done))<br>
(test::assert-equal '(x y z) (read tst-file :done))<br>
(test::assert-equal :done (read tst-file :done))<br>
(close tst-file)<br>
(test::assert-equal '(4 5 6) (read "(4 5 6)"))<br>
(def test-str "7 8 9")<br>
(test::assert-equal 7 (read test-str))<br>
(test::assert-equal 8 (read test-str))<br>
(test::assert-equal 9 (read test-str))<br>
(test::assert-error (read test-str))<br>
(def test-str "7 8 9")<br>
(test::assert-equal 7 (read test-str :done))<br>
(test::assert-equal 8 (read test-str :done))<br>
(test::assert-equal 9 (read test-str :done))<br>
(test::assert-equal :done (read test-str :done))<br>
(test::assert-equal '(x y z) (read "(x y z)"))<br>
<br>
</code>
</details>
<br>


| <a id="root::read-all" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``read-all``](#root::read-all-contents) | Type: Function |
 ``root::read-all`` | ``Usage: (read-all [file\|string]? empty-exp?) -> list\|vec\|empty-exp`` |

<span style="padding-left: 5px">Read a file or string and return the list representation.  This reads the entire
file or string and will wrap in an outer vector if not a vector or list (always
returns a vector or list).
Unlike most lisp readers this one will put loose symbols in a list (i.e. you
enter things at the repl without the enclosing parens).
Note the file|string arg is optional, if not provided will read from stdin (or
can provide stdin).
If the read item is empty (including a comment) then raises an error or produces
empty-exp if it is provided.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line tst-file "(1 2 3)(x y z)")<br>
(flush tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal '#((1 2 3)(x y z)) (read-all tst-file))<br>
(close tst-file)<br>
(test::assert-equal '(4 5 6) (read-all "(4 5 6)"))<br>
(test::assert-equal '(7 8 9) (read-all "7 8 9"))<br>
(test::assert-equal '(x y z) (read-all "(x y z)" :not-used))<br>
(test::assert-equal :empty (read-all ";(x y z)" :empty))<br>
<br>
</code>
</details>
<br>


| <a id="root::read-line" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``read-line``](#root::read-line-contents) | Type: Function |
 ``root::read-line`` | ``Usage: (read-line file) -> string`` |

<span style="padding-left: 5px">Read a line from a file.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line tst-file "Test Line Read Line One")<br>
(write-string tst-file "Test Line Read Line Two")<br>
(flush tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal "Test Line Read Line One<br>
" (read-line tst-file))<br>
(test::assert-equal "Test Line Read Line Two" (read-line tst-file))<br>
(close tst-file)<br>
<br>
</code>
</details>
<br>


| <a id="root::write-line" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``write-line``](#root::write-line-contents) | Type: Function |
 ``root::write-line`` | ``Usage: (write-line file string)`` |

<span style="padding-left: 5px">Write a line to a file.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-line tst-file "Test Line Write Line")<br>
(flush tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal "Test Line Write Line<br>
" (read-line tst-file))<br>
(close tst-file)<br>
<br>
</code>
</details>
<br>


| <a id="root::write-string" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``write-string``](#root::write-string-contents) | Type: Function |
 ``root::write-string`` | ``Usage: (write-string file string)`` |

<span style="padding-left: 5px">Write a string to a file.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :create :truncate))<br>
(write-string tst-file "Test Line Write String")<br>
(flush tst-file)<br>
(def tst-file (open "/tmp/slsh-tst-open.txt" :read))<br>
(test::assert-equal "Test Line Write String" (read-line tst-file))<br>
(close tst-file)<br>
<br>
</code>
</details>
<br>
### <a id="globals-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Globals forms](#globals-contents)
In sl-sh global symbols (made
by 'def) are wrapped in *earmuffs* like in common lisp. Some of these symbols
contain information used by the standard library and may be useful to
end users, while others are intended for use in scripting.


| <a id="root::*last-command*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*last-command*``](#root::*last-command*-contents) | Type: String |
 ``root::*last-command*`` | ``nil`` |

<span style="padding-left: 5px">Namespace: root
last run command by sl-sh on repl
</span>
<br>
<br>


| <a id="root::*last-status*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*last-status*``](#root::*last-status*-contents) | Type: Int |
 ``root::*last-status*`` | ``nil`` |

<span style="padding-left: 5px">Namespace: root
Return code of last run sl-sh command on the repl
</span>
<br>
<br>


| <a id="root::*repl-settings*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*repl-settings*``](#root::*repl-settings*-contents) | Type: HashMap |
 ``root::*repl-settings*`` | ``nil`` |

<span style="padding-left: 5px">Namespace: root
hash map of repl settings
</span>
<br>
<br>


| <a id="root::*std-lib-namespaces*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*std-lib-namespaces*``](#root::*std-lib-namespaces*-contents) | Type: Vector |
 ``root::*std-lib-namespaces*`` | ``nil`` |

<span style="padding-left: 5px">Namespace: root
vector of all namespaces present in sl-sh by default
</span>
<br>
<br>


| <a id="root::*std-lib-syms-hash*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*std-lib-syms-hash*``](#root::*std-lib-syms-hash*-contents) | Type: HashMap |
 ``root::*std-lib-syms-hash*`` | ``nil`` |

<span style="padding-left: 5px">Namespace: root
vector of all symbols in sl-sh-standard library
</span>
<br>
<br>
### <a id="hashmap-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Hashmap forms](#hashmap-contents)



| <a id="root::hash-clear!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-clear!``](#root::hash-clear!-contents) | Type: Function |
 ``root::hash-clear!`` | ``Usage: (hash-clear! hashmap)`` |

<span style="padding-left: 5px">Clears a hashmap.  This is a destructive form!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three")(#\S . "val S"))))<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-true (hash-haskey tst-hash :key1))<br>
(test::assert-true (hash-haskey tst-hash 'key2))<br>
(test::assert-true (hash-haskey tst-hash "key3"))<br>
(test::assert-true (hash-haskey tst-hash #\S))<br>
(hash-clear! tst-hash)<br>
(test::assert-equal 0 (length (hash-keys tst-hash)))<br>
(test::assert-false (hash-haskey tst-hash :key1))<br>
(test::assert-false (hash-haskey tst-hash 'key2))<br>
(test::assert-false (hash-haskey tst-hash "key3"))<br>
(test::assert-false (hash-haskey tst-hash #\S))<br>
<br>
</code>
</details>
<br>


| <a id="root::hash-get" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-get``](#root::hash-get-contents) | Type: SpecialForm |
 ``root::hash-get`` | ``Usage: (hash-get hashmap key default?) -> value`` |

<span style="padding-left: 5px">Get a value for a key from a hashmap.  If the optional default is provided and
the key is not in the hash then evaluate and return it.
NOTE: default will only be evaluted if it is used.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three")(#\S . "val S"))))<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :key1))<br>
(test::assert-equal "val two" (hash-get tst-hash 'key2))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(test::assert-equal "val S" (hash-get tst-hash #\S))<br>
(test::assert-equal "default" (hash-get tst-hash :not-here "default"))<br>
(test::assert-equal "string default" (hash-get tst-hash :not-here (str "string " "default")))<br>
<br>
</code>
</details>
<br>


| <a id="root::hash-haskey" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-haskey``](#root::hash-haskey-contents) | Type: Function |
 ``root::hash-haskey`` | ``Usage: (hash-haskey hashmap key)`` |

<span style="padding-left: 5px">Checks if a key is in a hashmap.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three")(#\S . "val S"))))<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-true (hash-haskey tst-hash :key1))<br>
(test::assert-true (hash-haskey tst-hash 'key2))<br>
(test::assert-true (hash-haskey tst-hash "key3"))<br>
(test::assert-true (hash-haskey tst-hash #\S))<br>
(test::assert-false (hash-haskey tst-hash 'key1))<br>
(test::assert-false (hash-haskey tst-hash :key2))<br>
(test::assert-false (hash-haskey tst-hash "keynone"))<br>
(hash-remove! tst-hash :key1)<br>
(test::assert-false (hash-haskey tst-hash :key1))<br>
(hash-set! tst-hash :key1 "val one b")<br>
(test::assert-true (hash-haskey tst-hash :key1))<br>
<br>
</code>
</details>
<br>


| <a id="root::hash-keys" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-keys``](#root::hash-keys-contents) | Type: Function |
 ``root::hash-keys`` | ``Usage: (hash-keys hashmap)`` |

<span style="padding-left: 5px">Returns a vector of all the hashmaps keys.  The keys will be unordered.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three")(#\S . "val S"))))<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-true (in? (hash-keys tst-hash) :key1) " Test :key1")<br>
(test::assert-true (in? (hash-keys tst-hash) 'key2) " Test key2")<br>
; Note string or char used as a key will be a symbol in the hash-keys list...<br>
(test::assert-true (in? (hash-keys tst-hash) 'S) " Test S")<br>
(test::assert-true (in? (hash-keys tst-hash) 'key3) " Test key3")<br>
(test::assert-false (in? (hash-keys tst-hash) :key4))<br>
<br>
</code>
</details>
<br>


| <a id="root::hash-remove!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-remove!``](#root::hash-remove!-contents) | Type: Function |
 ``root::hash-remove!`` | ``Usage: (hash-remove! hashmap key)`` |

<span style="padding-left: 5px">Remove a key from a hashmap.  This is a destructive form!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three")(#\S . "val S"))))<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :key1))<br>
(test::assert-equal "val two" (hash-get tst-hash 'key2))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(test::assert-equal "val S" (hash-get tst-hash #\S))<br>
(hash-remove! tst-hash 'key2)<br>
(test::assert-equal 3 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :key1))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(test::assert-equal "val S" (hash-get tst-hash #\S))<br>
(hash-remove! tst-hash :key1)<br>
(test::assert-equal 2 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(test::assert-equal "val S" (hash-get tst-hash #\S))<br>
(hash-remove! tst-hash "key3")<br>
(test::assert-equal 1 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val S" (hash-get tst-hash #\S))<br>
(hash-remove! tst-hash #\S)<br>
(test::assert-equal 0 (length (hash-keys tst-hash)))<br>
<br>
</code>
</details>
<br>


| <a id="root::hash-set!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash-set!``](#root::hash-set!-contents) | Type: Function |
 ``root::hash-set!`` | ``Usage: (hash-set! hashmap key value)`` |

<span style="padding-left: 5px">Add or update a hashmap key's value.  This is a destructive form!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash))<br>
(test::assert-equal 0 (length (hash-keys tst-hash)))<br>
(hash-set! tst-hash :new-key '(1 2 3))<br>
(test::assert-equal 1 (length (hash-keys tst-hash)))<br>
(test::assert-equal '(1 2 3) (hash-get tst-hash :new-key))<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three"))))<br>
(test::assert-equal 3 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :key1))<br>
(test::assert-equal "val two" (hash-get tst-hash 'key2))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(hash-set! tst-hash :new-key '(1 2 3))<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :key1))<br>
(test::assert-equal "val two" (hash-get tst-hash 'key2))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(test::assert-equal '(1 2 3) (hash-get tst-hash :new-key))<br>
(hash-set! tst-hash 'key2 "val two b")<br>
(hash-set! tst-hash :key1 "val one b")<br>
(hash-set! tst-hash "key3" "val three b")<br>
(test::assert-equal 4 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one b" (hash-get tst-hash :key1))<br>
(test::assert-equal "val two b" (hash-get tst-hash 'key2))<br>
(test::assert-equal "val three b" (hash-get tst-hash "key3"))<br>
(test::assert-equal '(1 2 3) (hash-get tst-hash :new-key))<br>
<br>
</code>
</details>
<br>


| <a id="root::make-hash" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``make-hash``](#root::make-hash-contents) | Type: Function |
 ``root::make-hash`` | ``Usage: (make-hash associations?)`` |

<span style="padding-left: 5px">Make a new hash map.
If associations is provided (makes an empty map if not) then it is a list of
pairs (key . value) that populate the intial map.  Neither key nor value in the
associations will be evaluated.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-hash (make-hash))<br>
(test::assert-equal 0 (length (hash-keys tst-hash)))<br>
(def tst-hash (make-hash ()))<br>
(test::assert-equal 0 (length (hash-keys tst-hash)))<br>
(def tst-hash (make-hash nil))<br>
(test::assert-equal 0 (length (hash-keys tst-hash)))<br>
(def tst-hash (make-hash '((:key1 . "val one")(key2 . "val two")("key3" . "val three"))))<br>
(test::assert-equal 3 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :key1))<br>
(test::assert-equal "val two" (hash-get tst-hash 'key2))<br>
(test::assert-equal "val three" (hash-get tst-hash "key3"))<br>
(def tst-hash (make-hash '#((:keyv1 . "val one")(keyv2 . "val two")("keyv3" . "val three"))))<br>
(test::assert-equal 3 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :keyv1))<br>
(test::assert-equal "val two" (hash-get tst-hash 'keyv2))<br>
(test::assert-equal "val three" (hash-get tst-hash "keyv3"))<br>
; Not in test below that tst-hash-val is NOT evaluated so the symbol is the value.<br>
(def tst-hash-val "some val")<br>
(def tst-hash (make-hash '#((:keyv1 . "val one")(:keyv2 . "val two")(:keyv3 . tst-hash-val))))<br>
(test::assert-equal 3 (length (hash-keys tst-hash)))<br>
(test::assert-equal "val one" (hash-get tst-hash :keyv1))<br>
(test::assert-equal "val two" (hash-get tst-hash :keyv2))<br>
(test::assert-equal 'tst-hash-val (hash-get tst-hash :keyv3))<br>
<br>
</code>
</details>
<br>
### <a id="iterator-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Iterator forms](#iterator-contents)



| <a id="iterator::append" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``append``](#iterator::append-contents) | Type: Lambda |
 ``iterator::append`` | ``Usage: (append first-iter &rest rest-iters)`` |

<span style="padding-left: 5px">Combine the provided items into a single iterator (calls iter on each parameter).
If non-list items are passed they are wrapped in a singleton iterator (i.e. will
work with loose object).  Note that nil is an empty list not a "loose item".
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter (append '(0 1 2) '#(3 4 5) '(6 7 8 9)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter (append '(0 1 2) 3 4 5 '(6 7 8 9)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter (append 0 1 2 '(3 4)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 0 (test-iter :next!))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 2 (test-iter :next!))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-equal 4 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter (append 0 1 2 nil))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 0 (test-iter :next!))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 2 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter (append 0 1 2 '(nil)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 0 (test-iter :next!))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 2 (test-iter :next!))<br>
(assert-equal nil (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::append-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``append-iter``](#iterator::append-iter-contents) | Type: Lambda |
 ``iterator::append-iter`` | ``Usage: (append-iter)`` |

<span style="padding-left: 5px">Iterator that appends multiple iterators.  Append iter will consume
the iterators it is appending.  If non-list items are passed they are wrapped in
a singleton iterator (i.e. will work with loose object).
attribute: iters private
method: :next!
method: :empty?
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter ((iterator::append-iter) :init '(0 1 2) '#(3 4 5) '(6 7 8 9)))<br>
(def test-slice-iter (test-iter :slice 3 7))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 3 (test-slice-iter :next!))<br>
(assert-equal 4 (test-slice-iter :next!))<br>
(assert-equal 5 (test-slice-iter :next!))<br>
(assert-equal 6 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-iter ((iterator::append-iter) :init '(0 1 2) nil '#(3 4 5) nil '(6 7 8 9)))<br>
(def test-slice-iter (test-iter :slice 0 4))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 0 (test-slice-iter :next!))<br>
(assert-equal 1 (test-slice-iter :next!))<br>
(assert-equal 2 (test-slice-iter :next!))<br>
(assert-equal 3 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-iter ((iterator::append-iter) :init '(0 1 2) '#(3 4 5) '(6 7 8 9)))<br>
(def test-slice-iter (test-iter :slice 7))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 7 (test-slice-iter :next!))<br>
(assert-equal 8 (test-slice-iter :next!))<br>
(assert-equal 9 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-iter ((iterator::append-iter) :init '(0 1 2) '#(3 4 5) '(6 7 8 9)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter ((iterator::append-iter) :init '(0 1 2) '() '#(3 4 5) nil '(6 7 8 9)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter ((iterator::append-iter) :init nil '(0 1 2) (vec) '#(3 4 5) '(6 7 8 9) nil))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::append-to!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``append-to!``](#iterator::append-to!-contents) | Type: Lambda |
 ``iterator::append-to!`` | ``Usage: (append-to! ret &rest others)`` |

<span style="padding-left: 5px">Combine the provided items after the first (first must be a vector or list)
into a single iterator.  These values are added the first argument destructively.
If non-list items are passed they are wrapped in a singleton iterator (i.e. will
work with loose object).  Note that nil is an empty list not a "loose item".
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter '(0 1 2))<br>
(append-to! test-iter '#(3 4 5) '(6 7 8 9))<br>
(set! test-iter (iter test-iter))<br>
(def test-slice-iter (test-iter :slice 3 7))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 3 (test-slice-iter :next!))<br>
(assert-equal 4 (test-slice-iter :next!))<br>
(assert-equal 5 (test-slice-iter :next!))<br>
(assert-equal 6 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-iter '(0 1 2))<br>
(append-to! test-iter '#(3 4 5) '(6 7 8 9))<br>
(set! test-iter (iter test-iter))<br>
(def test-slice-iter (test-iter :slice 0 4))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 0 (test-slice-iter :next!))<br>
(assert-equal 1 (test-slice-iter :next!))<br>
(assert-equal 2 (test-slice-iter :next!))<br>
(assert-equal 3 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-iter '(0 1 2))<br>
(append-to! test-iter '#(3 4 5) '(6 7 8 9))<br>
(set! test-iter (iter test-iter))<br>
(def test-slice-iter (test-iter :slice 7))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 7 (test-slice-iter :next!))<br>
(assert-equal 8 (test-slice-iter :next!))<br>
(assert-equal 9 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-iter '(0 1 2))<br>
(append-to! test-iter '#(3 4 5) '(6 7 8 9))<br>
(set! test-iter (iter test-iter))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter nil)<br>
(append-to! test-iter nil '(0 1 2) nil '#(3 4 5) '(6 7 8 9) nil)<br>
(set! test-iter (iter test-iter))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 10 (test-iter :count))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter (vec 0))<br>
(append-to! test-iter nil '#(1) '(2 3) nil)<br>
(set! test-iter (iter test-iter))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 0 (test-iter :next!))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 2 (test-iter :next!))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::collect" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect``](#iterator::collect-contents) | Type: Lambda |
 ``iterator::collect`` | ``Usage: (collect s)`` |

<span style="padding-left: 5px">Collect all the values into a list.  This will consume the iterator and
produce a new list.  Will call iter on input to turn a collection into an iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def collect-test (iterator::collect '#(1 2 3)))<br>
(assert-true (list? collect-test))<br>
(assert-equal '(1 2 3) collect-test)<br>
<br>
</code>
</details>
<br>


| <a id="iterator::collect-str" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect-str``](#iterator::collect-str-contents) | Type: Lambda |
 ``iterator::collect-str`` | ``Usage: (collect-str s)`` |

<span style="padding-left: 5px">Collect all the values into a string.  This will consume the iterator and
produce a new string.  Will call iter on input to turn a collection into an iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def collect-str-test (iterator::collect-str (iterator::map (fn (ch) (char-upper ch)) "λabc σ")))<br>
(assert-true (string? collect-str-test))<br>
(assert-equal "ΛABC Σ" collect-str-test)<br>
<br>
</code>
</details>
<br>


| <a id="iterator::collect-vec" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect-vec``](#iterator::collect-vec-contents) | Type: Lambda |
 ``iterator::collect-vec`` | ``Usage: (collect-vec s)`` |

<span style="padding-left: 5px">Collect all the values into a vector.  This will consume the iterator and
produce a new vector.  Will call iter on input to turn a collection into an iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def collect-vec-test (iterator::collect-vec '(1 2 3)))<br>
(assert-true (vec? collect-vec-test))<br>
(assert-equal '#(1 2 3) collect-vec-test)<br>
<br>
</code>
</details>
<br>


| <a id="iterator::double-ended-iter?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``double-ended-iter?``](#iterator::double-ended-iter?-contents) | Type: Lambda |
 ``iterator::double-ended-iter?`` | ``Usage: (double-ended-iter? thing)`` |

<span style="padding-left: 5px">Return true if thing is an iterator and double ended, nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(struct::defstruct test-iter<br>
; fields<br>
(current 0)<br>
; methods<br>
(:fn next! (self) (do (def val current)(set! current (+ 1 current)) val))<br>
(:fn empty? (self) (>= current 3))<br>
(:impl iterator::iterator))<br>
(assert-true (iterator::double-ended-iter? (iterator::iter '(1 2 3))))<br>
(assert-false (iterator::double-ended-iter? '(1 2 3)))<br>
(assert-false (iterator::double-ended-iter? (test-iter)))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::double-ended-iterator" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``double-ended-iterator``](#iterator::double-ended-iterator-contents) | Type: Lambda |
 ``iterator::double-ended-iterator`` | ``Usage: (defstruct iter (:fn next! (self)...)(:fn next-back! (self)...)(:fn empty? (self)...)(:impl iterator::iterator iterator::double-ended-iterator))`` |

<span style="padding-left: 5px">Trait that makes an iterator double ended (can get items from front and back.
Requires a struct to define methods next-back! and implement iterator.
Note that next! and next-back! can not cross, the iterator is empty when they meet.
method: :nth-back!
Consume the iterator until the nth element from the end and return it (0 based).
Note that repeated called to nth-back! will return new data since it consumes the iterator.
method: :reverse
Produce an iterator that is the reverse of self.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'struct)<br>
(ns-import 'iterator)<br>
(defstruct test-double-iter<br>
; fields<br>
(current 0)<br>
(current-end 2)<br>
; methods<br>
(:fn next! (self) (do (var val current)(set! current (+ 1 current)) val))<br>
(:fn next-back! (self) (do (var val current-end)(set! current-end (- current-end 1)) val))<br>
(:fn empty? (self) (> current current-end))<br>
(:impl iterator::iterator iterator::double-ended-iterator))<br>
(def tmap (test-double-iter))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 0 (tmap :next!))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
(def tmap (test-double-iter))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 2 (tmap :next-back!))<br>
(assert-equal 1 (tmap :next-back!))<br>
(assert-equal 0 (tmap :next-back!))<br>
(assert-true (tmap :empty?))<br>
(def tmap (test-double-iter))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 0 (tmap :next!))<br>
(assert-equal 2 (tmap :next-back!))<br>
(assert-equal 1 (tmap :next-back!))<br>
(assert-true (tmap :empty?))<br>
; :nth-back! Example<br>
(def tmap ((vec-iter) :init '#(0 1 2 3 4) 0))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 4 (tmap :nth-back! 0))<br>
(assert-equal 3 (tmap :nth-back! 0))<br>
(assert-equal 0 (tmap :nth-back! 2))<br>
(assert-true (tmap :empty?))<br>
; :reverse Example<br>
(def tmap ((test-double-iter) :reverse))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-equal 0 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::empty?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``empty?``](#iterator::empty?-contents) | Type: Lambda |
 ``iterator::empty?`` | ``Usage: (empty? s)`` |

<span style="padding-left: 5px">Is an iterator empty (no more items)?  Will call iter on input first.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (iterator::empty? nil))<br>
(assert-true (iterator::empty? '#()))<br>
(assert-false (iterator::empty? '#(1)))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::file-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``file-iter``](#iterator::file-iter-contents) | Type: Lambda |
 ``iterator::file-iter`` | ``Usage: (file-iter)`` |

<span style="padding-left: 5px">Iterator that wraps a file.  Each call to next! returns the next line (with
trailing newline.
attribute: file private
attribute: next-line private
method: :next!
method: :empty?
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-file (open "/tmp/file-iter-test.txt" :create :truncate))<br>
(write-line tst-file "line 1")<br>
(write-line tst-file "line 2")<br>
(write-line tst-file "line 3")<br>
(write-string tst-file "line 4")<br>
(close tst-file)<br>
(def test-iter ((iterator::file-iter) :init (open "/tmp/file-iter-test.txt")))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal "line 1\n" (test-iter :next!))<br>
(assert-equal "line 2\n" (test-iter :next!))<br>
(assert-equal "line 3\n" (test-iter :next!))<br>
(assert-equal "line 4" (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::filter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``filter``](#iterator::filter-contents) | Type: Lambda |
 ``iterator::filter`` | ``Usage: (filter predicate items)`` |

<span style="padding-left: 5px">Returns a filter-iter around items (will call iter on items).
Iterator that applies a lambda to each element to determine if is returned- is lazy.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter (iterator::filter (fn (x) (not (= x 2))) '(1 2 3)))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::filter-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``filter-iter``](#iterator::filter-iter-contents) | Type: Lambda |
 ``iterator::filter-iter`` | ``Usage: (filter-iter)`` |

<span style="padding-left: 5px">Iterator that applies a lambda to each element to determine if is returned- is lazy.
attribute: data private
attribute: predicate private
attribute: next private
attribute: is-empty private
method: :next!
method: :empty?
method: :advance-data!
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter (((iterator::list-iter) :init '(1 2 3)) :filter (fn (x) (not (= x 2)))))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::for" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``for``](#iterator::for-contents) | Type: Macro |
 ``iterator::for`` | ``Usage: (for bind in items body)`` |

<span style="padding-left: 5px">Loops over each element in an iterator.  Will call iter on the input object.
bind is bound to the current element of items and is accesible
in body. body is evaluated a number of times equal to the the number of items
in in_list.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def i 0)<br>
(iterator::for x in (iterator::range 11) (set! i (+ 1 i)))<br>
(assert-equal 11 i)<br>
<br>
</code>
</details>
<br>


| <a id="iterator::for-i" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``for-i``](#iterator::for-i-contents) | Type: Macro |
 ``iterator::for-i`` | ``Usage: (for-i idx-bind bind in items body)`` |

<span style="padding-left: 5px">Loops over each element in an iterator.  Will call iter on the input object.
idx-bind is bound to an incrementing number starting with 0.
bind is bound to the current element of items and is accesible
in body. body is evaluated a number of times equal to the the number of items
in in_list.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def i 0)<br>
(def i-tot 0)<br>
(for-i idx x in '(1 2 3 4 5 6 7 8 9 10 11) (do (set! i-tot (+ idx i-tot))(set! i (+ 1 i))))<br>
(assert-equal 11 i)<br>
(assert-equal 55 i-tot)<br>
<br>
</code>
</details>
<br>


| <a id="iterator::interleave" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``interleave``](#iterator::interleave-contents) | Type: Lambda |
 ``iterator::interleave`` | ``Usage: (interleave fst scnd)`` |

<span style="padding-left: 5px">interleaves two iterators together. Resultant iter
is double length of fst unless scnd has less items. Then iter is double length
of scnd.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'iterator)<br>
(test::assert-equal (list 1 2 3 4) (collect (interleave (iter (list 1 3)) (iter (list 2 4)))))<br>
(test::assert-equal (list 1 2 3 4) (collect (interleave (iter (list 1 3)) (iter (list 2 4 5)))))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::interleave-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``interleave-iter``](#iterator::interleave-iter-contents) | Type: Lambda |
 ``iterator::interleave-iter`` | ``Usage: (interleave-iter)`` |

<span style="padding-left: 5px">create iterator that interleaves two iterators together. Resultant iter
is double length of fst if fst has less items, or double length of scnd if
scnd has less items.
attribute: fst private
attribute: scnd private
attribute: flip-flop private
method: :next!
method: :empty?
method: :init
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
<br>
</code>
</details>
<br>


| <a id="iterator::iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``iter``](#iterator::iter-contents) | Type: Lambda |
 ``iterator::iter`` | ``Usage: (iter thing)`` |

<span style="padding-left: 5px">Return thing as an iterator if possible (if it is an iterator just return thing).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (iterator::iter? (iterator::iter '(1 2 3))))<br>
(assert-true (iterator::iter? (iterator::iter '#(1 2 3))))<br>
(assert-true (iterator::iter? (iterator::iter "abc")))<br>
(assert-true (iterator::iter? (iterator::iter (iterator::iter '(1 2 3)))))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::iter?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``iter?``](#iterator::iter?-contents) | Type: Lambda |
 ``iterator::iter?`` | ``Usage: (iter? thing)`` |

<span style="padding-left: 5px">Return true if thing is an iterator, nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (iterator::iter? (iterator::iter '(1 2 3))))<br>
(assert-false (iterator::iter? '(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::iterator" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``iterator``](#iterator::iterator-contents) | Type: Lambda |
 ``iterator::iterator`` | ``Usage: (defstruct iter (:fn next! (self)...)(:fn empty? (self)...)(:impl iterator::iterator))`` |

<span style="padding-left: 5px">Trait that provides iterator methods.
Requires a struct to define methods next! and empty?
method: :collect
Collect all the values into a list.  This will consume the iterator and
produce a new list.
method: :collect-vec
Collect all the values into a vector.  This will consume the iterator and
produce a new list.
method: :collect-str
Collect all the values into a string.  This will consume the iterator and
produce a new string.
method: :map
Apply the provided function to each element of the iterator.  Map is lazy.
method: :filter
Apply the provided predicate to the iterator producing only elements that are true.  Filter is lazy.
method: :slice
method: :count
Consume the iterator and return the number of items.
method: :nth!
Consume the iterator until the nth! element and return it (0 based).
Note that repeated called to nth! will return new data since it consumes the iterator.
method: :double-ended?
Return t if this iterator is double ended, nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'struct)<br>
(ns-import 'iterator)<br>
(defstruct test-iter<br>
; fields<br>
(current 0)<br>
; methods<br>
(:fn next! (self) (do (def val current)(set! current (+ 1 current)) val))<br>
(:fn empty? (self) (>= current 3))<br>
(:impl iterator::iterator))<br>
(def tmap (test-iter))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 0 (tmap :next!))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
; :collect Example<br>
(def collect-test ((iter '#(1 2 3)) :collect))<br>
(assert-true (list? collect-test))<br>
(assert-equal '(1 2 3) collect-test)<br>
; :collect-vec Example<br>
(def collect-vec-test ((iter '(1 2 3)) :collect-vec))<br>
(assert-true (vec? collect-vec-test))<br>
(assert-equal '#(1 2 3) collect-vec-test)<br>
; :collect-str Example<br>
(def collect-str-test (((iter "λabc σ") :map (fn (ch) (char-upper ch))) :collect-str))<br>
(assert-true (string? collect-str-test))<br>
(assert-equal "ΛABC Σ" collect-str-test)<br>
; :map Example<br>
(def tmap ((test-iter) :map (fn (x) (+ 1 x))))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-equal 3 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
; :filter Example<br>
(def tmap ((test-iter) :filter (fn (x) (not (= x 1)))))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 0 (tmap :next!))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
; :count Example<br>
(def tmap (test-iter))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 3 (tmap :count))<br>
(assert-true (tmap :empty?))<br>
; :nth! Example<br>
(def tmap ((list-iter) :init '(0 1 2 3 4)))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 0 (tmap :nth! 0))<br>
(assert-equal 1 (tmap :nth! 0))<br>
(assert-equal 4 (tmap :nth! 2))<br>
(assert-true (tmap :empty?))<br>
; :double-ended? Example<br>
(ns-import 'struct)<br>
(ns-import 'iterator)<br>
(defstruct test-double-iter<br>
; fields<br>
(current 0)<br>
(current-end 2)<br>
; methods<br>
(:fn next! (self) (do (def val current)(set! current (+ 1 current)) val))<br>
(:fn next-back! (self) (do (def val current-end)(set! current-end (- current-end 1)) val))<br>
(:fn empty? (self) (> current current-end))<br>
(:impl iterator::iterator iterator::double-ended-iterator))<br>
(assert-false ((test-iter) :double-ended?))<br>
(assert-true ((test-double-iter) :double-ended?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::list-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``list-iter``](#iterator::list-iter-contents) | Type: Lambda |
 ``iterator::list-iter`` | ``Usage: (list-iter)`` |

<span style="padding-left: 5px">Iterator that wraps a list.
attribute: data private
attribute: rev-data private
attribute: elements private
method: :next!
method: :next-back!
method: :empty?
method: :init
method: :make-rev
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-list-iter ((list-iter) :init '(1 2 3)))<br>
(assert-false (test-list-iter :empty?))<br>
(assert-equal 1 (test-list-iter :next!))<br>
(assert-equal 2 (test-list-iter :next!))<br>
(assert-equal 3 (test-list-iter :next!))<br>
(assert-true (test-list-iter :empty?))<br>
(def test-list-iter ((list-iter) :init '(1 2 3)))<br>
(assert-false (test-list-iter :empty?))<br>
(assert-equal 1 (test-list-iter :next!))<br>
(assert-equal 3 (test-list-iter :next-back!))<br>
(assert-equal 2 (test-list-iter :next!))<br>
(assert-true (test-list-iter :empty?))<br>
(def test-list-iter ((list-iter) :init '(1 2 3)))<br>
(assert-false (test-list-iter :empty?))<br>
(assert-equal 3 (test-list-iter :next-back!))<br>
(assert-equal 2 (test-list-iter :next-back!))<br>
(assert-equal 1 (test-list-iter :next-back!))<br>
(assert-true (test-list-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::map" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``map``](#iterator::map-contents) | Type: Lambda |
 ``iterator::map`` | ``Usage: (map map-fn items)`` |

<span style="padding-left: 5px">Returns a map-iter around items (will call iter on items).
Apply the provided function to each element of the iterator.  Map is lazy.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tmap (iterator::map (fn (x) (+ 1 x)) '(0 1 2)))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-equal 3 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
(def tmap (iterator::reverse (iterator::map (fn (x) (+ 1 x)) '(0 1 2))))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 3 (tmap :next!))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::map-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``map-iter``](#iterator::map-iter-contents) | Type: Lambda |
 ``iterator::map-iter`` | ``Usage: (map-iter)`` |

<span style="padding-left: 5px">Iterator that applies a lambda to each element of another iterator- is lazy.
attribute: data private
attribute: map-fn private
method: :next!
method: :next-back!
method: :empty?
method: :init
method: :double-ended?
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-map-iter (((iterator::list-iter) :init '(1 2 3)) :map (fn (x) (* x 2))))<br>
(assert-false (test-map-iter :empty?))<br>
(assert-equal 2 (test-map-iter :next!))<br>
(assert-equal 4 (test-map-iter :next!))<br>
(assert-equal 6 (test-map-iter :next!))<br>
(assert-true (test-map-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::meld" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meld``](#iterator::meld-contents) | Type: Lambda |
 ``iterator::meld`` | ``Usage: (meld fst scnd)`` |

<span style="padding-left: 5px">melds two iterators together. Resultant iter is composed of pairs (fst, scnd)
for each value of next! in each provided iterator. Length of returned
iter is equal to length of the shortest provided iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'iterator)<br>
(test::assert-equal (list (join 'string 'bean) (join 'monte 'carlo)) (collect (meld (iter (list 'string 'monte)) (iter (list 'bean 'carlo)))))<br>
(test::assert-equal (list (join 1 2) (join 3 4)) (collect (meld (iter (list 1 3)) (iter (list 2 4 5)))))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::meld-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``meld-iter``](#iterator::meld-iter-contents) | Type: Lambda |
 ``iterator::meld-iter`` | ``Usage: (meld-iter)`` |

<span style="padding-left: 5px">create iterator that melds two iterators together. Resultant iter
is composed of pairs (fst, scnd) for each value of next! in each provided
iterator. Length of meld-iter is equal to length of shorter iterator.
attribute: fst private
attribute: scnd private
attribute: flip-flop private
method: :next!
method: :empty?
method: :init
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
<br>
</code>
</details>
<br>


| <a id="iterator::next!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``next!``](#iterator::next!-contents) | Type: Lambda |
 ``iterator::next!`` | ``Usage: (next! s)`` |

<span style="padding-left: 5px">Calls iter on s and returns the next item.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 1 (iterator::next! '(1 2 3)))<br>
(assert-equal 1 (iterator::next! '#(1 2 3)))<br>
(def next-test (iterator::iter '(4 5 6)))<br>
(assert-equal 4 (iterator::next! next-test))<br>
(assert-equal 5 (iterator::next! next-test))<br>
(assert-equal 6 (iterator::next! next-test))<br>
(assert-true (next-test :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::nth" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``nth``](#iterator::nth-contents) | Type: Lambda |
 ``iterator::nth`` | ``Usage: (nth idx coll)`` |

<span style="padding-left: 5px">Consume the iterator until the idx (nth) element and return it (0 based).
Note that repeated called to nth will return new data since it consumes the iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tmap ((list-iter) :init '(0 1 2 3 4)))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 0 (nth 0 tmap))<br>
(assert-equal 1 (nth 0 tmap))<br>
(assert-equal 4 (nth 2 tmap))<br>
(assert-true (tmap :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::range" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``range``](#iterator::range-contents) | Type: Lambda |
 ``iterator::range`` | ``Usage: (range &rest i)`` |

<span style="padding-left: 5px">Create an iterator that generates numbers within a range.
Can be called with one int (n) to produce 0..(n-1) or with two ints (m, n) to
produce m..n.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter (iterator::range 3 6))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-equal 4 (test-iter :next!))<br>
(assert-equal 5 (test-iter :next!))<br>
(assert-equal 6 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter (iterator::range 3))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 0 (test-iter :next!))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-equal 2 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::range-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``range-iter``](#iterator::range-iter-contents) | Type: Lambda |
 ``iterator::range-iter`` | ``Usage: (range-iter)`` |

<span style="padding-left: 5px">Iterator that generates numbers within a range.
attribute: start private
attribute: end private
method: :next!
method: :next-back!
method: :empty?
method: :init
method: :count
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter ((range-iter) :init 3 6))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-equal 4 (test-iter :next!))<br>
(assert-equal 5 (test-iter :next!))<br>
(assert-equal 6 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
(def test-iter ((range-iter) :init 3 6))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 6 (test-iter :next-back!))<br>
(assert-equal 5 (test-iter :next-back!))<br>
(assert-equal 4 (test-iter :next-back!))<br>
(assert-equal 3 (test-iter :next-back!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::reduce" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reduce``](#iterator::reduce-contents) | Type: Lambda |
 ``iterator::reduce`` | ``Usage: (reduce reducing-fcn init-val coll)`` |

<span style="padding-left: 5px">reduce is used to amalgamate a provided iterator, coll, and an intitial value,
according to the reducing function provided. The iter function will be called
on coll to make sure it is an iterator. The reducing-fcn should be a function
of two arguments. In the first iteration of reduce, the init-val will be used as
the first argument to the reducing-fcn and (next! coll) will be used as the
second argument. For all subsequent iterations, The result from the previous
application of the reducing-fcn will be used as the first argument to the
reducing-fcn and the second argument will be the next item in the collection
when the collection is empty reduce will return the amalgamated result.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (= 15 (reduce + 0 (list 1 2 3 4 5))))<br>
(assert-false (= 15 (reduce + 1 (list 1 2 3 4 5))))<br>
(assert-true (= "one hoopy frood" (reduce str "" (list "one " "hoopy " "frood"))))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::reduce-times" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reduce-times``](#iterator::reduce-times-contents) | Type: Lambda |
 ``iterator::reduce-times`` | ``Usage: (reduce-times value wrapping-fcn times)`` |

<span style="padding-left: 5px">Apply wrapping-fcn to value number of times. Function is recursive. Recursive
binding for value is previous application of wrapping function to value.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal (list (list 3)) (reduce-times 3 list 2))<br>
(assert-equal 5 (reduce-times (reduce-times 5 list 5) first 5))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::repeat" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``repeat``](#iterator::repeat-contents) | Type: Lambda |
 ``iterator::repeat`` | ``Usage: (repeat target &rest n)`` |

<span style="padding-left: 5px">repeat target n times. if n is not provided returns infinite iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal (list #\m #\m #\m #\m) (collect (repeat #\m 4)))<br>
(test::assert-equal (list #\m #\m #\m #\m) (collect (take (repeat #\m) 4)))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::repeat-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``repeat-iter``](#iterator::repeat-iter-contents) | Type: Lambda |
 ``iterator::repeat-iter`` | ``Usage: (repeat-iter)`` |

<span style="padding-left: 5px">iterator that returns provided repeat with specified length. if length is
negative returns infinite iterator.
attribute: to-repeat private
attribute: len private
attribute: current private
method: :empty?
method: :next!
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
<br>
</code>
</details>
<br>


| <a id="iterator::reverse" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reverse``](#iterator::reverse-contents) | Type: Lambda |
 ``iterator::reverse`` | ``Usage: (reverse items)`` |

<span style="padding-left: 5px">Produce an iterator the is the reverse of items.  Will call iter on items and
requires a double ended iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tmap (reverse ((vec-iter) :init '#(0 1 2) 0)))<br>
(assert-false (tmap :empty?))<br>
(assert-equal 2 (tmap :next!))<br>
(assert-equal 1 (tmap :next!))<br>
(assert-equal 0 (tmap :next!))<br>
(assert-true (tmap :empty?))<br>
(assert-error (reverse "string"))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::reverse-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reverse-iter``](#iterator::reverse-iter-contents) | Type: Lambda |
 ``iterator::reverse-iter`` | ``Usage: (reverse-iter)`` |

<span style="padding-left: 5px">Iterator that reverses another iterators direction.  Requires a double ended iterator.
attribute: wrapped private
method: :next!
method: :next-back!
method: :empty?
method: :init
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter (((vec-iter) :init '#(1 2 3) 0) :reverse))<br>
(assert-false (test-iter :empty?))<br>
(assert-equal 3 (test-iter :next!))<br>
(assert-equal 2 (test-iter :next!))<br>
(assert-equal 1 (test-iter :next!))<br>
(assert-true (test-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::slice" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``slice``](#iterator::slice-contents) | Type: Lambda |
 ``iterator::slice`` | ``Usage: (slice items start &rest end)`` |

<span style="padding-left: 5px">Provides a slice of iterator.  Will call iter on items.  Slice iter will consume
the iterator it is slicing.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-slice-iter (slice '(0 1 2 3 4 5 6 7 8 9) 3 6))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 3 (test-slice-iter :next!))<br>
(assert-equal 4 (test-slice-iter :next!))<br>
(assert-equal 5 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-slice-iter (slice '(0 1 2 3 4 5 6 7 8 9) 0 3))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 0 (test-slice-iter :next!))<br>
(assert-equal 1 (test-slice-iter :next!))<br>
(assert-equal 2 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-slice-iter (slice '(0 1 2 3 4 5 6 7 8 9) 7))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 7 (test-slice-iter :next!))<br>
(assert-equal 8 (test-slice-iter :next!))<br>
(assert-equal 9 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::slice-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``slice-iter``](#iterator::slice-iter-contents) | Type: Lambda |
 ``iterator::slice-iter`` | ``Usage: (slice-iter)`` |

<span style="padding-left: 5px">Iterator that provides a slice of the underlying iter.  Slice iter will consume
the iterator it is slicing.
attribute: data private
attribute: start private
attribute: total private
attribute: count private
method: :next!
method: :empty?
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-slice-iter (((iterator::list-iter) :init '(0 1 2 3 4 5 6 7 8 9)) :slice 3 6))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 3 (test-slice-iter :next!))<br>
(assert-equal 4 (test-slice-iter :next!))<br>
(assert-equal 5 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-slice-iter (((iterator::list-iter) :init '(0 1 2 3 4 5 6 7 8 9)) :slice 0 3))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 0 (test-slice-iter :next!))<br>
(assert-equal 1 (test-slice-iter :next!))<br>
(assert-equal 2 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
(def test-slice-iter (((iterator::list-iter) :init '(0 1 2 3 4 5 6 7 8 9)) :slice 7))<br>
(assert-false (test-slice-iter :empty?))<br>
(assert-equal 7 (test-slice-iter :next!))<br>
(assert-equal 8 (test-slice-iter :next!))<br>
(assert-equal 9 (test-slice-iter :next!))<br>
(assert-true (test-slice-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::string-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``string-iter``](#iterator::string-iter-contents) | Type: Lambda |
 ``iterator::string-iter`` | ``Usage: (string-iter)`` |

<span style="padding-left: 5px">Iterator that wraps a string.
attribute: data private
method: :next!
method: :empty?
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-string-iter ((string-iter) :init "123"))<br>
(assert-false (test-string-iter :empty?))<br>
(assert-equal #\1 (test-string-iter :next!))<br>
(assert-equal #\2 (test-string-iter :next!))<br>
(assert-equal #\3 (test-string-iter :next!))<br>
(assert-true (test-string-iter :empty?))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::take" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``take``](#iterator::take-contents) | Type: Lambda |
 ``iterator::take`` | ``Usage: (take provided-iter n)`` |

<span style="padding-left: 5px">return iterator with first n items of provided-iter. Returned iterator is the
same as provided-iter if n is greater than the length of provided-iter.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal (list #\m #\m) (collect (take (repeat #\m 4) 2)))<br>
<br>
</code>
</details>
<br>


| <a id="iterator::take-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``take-iter``](#iterator::take-iter-contents) | Type: Lambda |
 ``iterator::take-iter`` | ``Usage: (take-iter)`` |

<span style="padding-left: 5px">take itertor
attribute: provided-iter private
attribute: len private
attribute: current private
method: :empty?
method: :next!
method: :init
impl iterator::iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
<br>
</code>
</details>
<br>


| <a id="iterator::vec-iter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-iter``](#iterator::vec-iter-contents) | Type: Lambda |
 ``iterator::vec-iter`` | ``Usage: (vec-iter)`` |

<span style="padding-left: 5px">Iterator that wraps a vector.
attribute: data private
attribute: start private
attribute: end private
method: :next!
method: :next-back!
method: :empty?
method: :nth!
method: :nth-back!
method: :init
impl iterator::iterator
impl iterator::double-ended-iterator
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-vec-iter ((vec-iter) :init '#(1 2 3) 0))<br>
(assert-false (test-vec-iter :empty?))<br>
(assert-equal 1 (test-vec-iter :next!))<br>
(assert-equal 2 (test-vec-iter :next!))<br>
(assert-equal 3 (test-vec-iter :next!))<br>
(assert-true (test-vec-iter :empty?))<br>
(def test-vec-iter ((vec-iter) :init (vec) 0))<br>
(assert-true (test-vec-iter :empty?))<br>
<br>
</code>
</details>
<br>
### <a id="logger-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Logger forms](#logger-contents)



| <a id="root::logger" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``logger``](#root::logger-contents) | Type: Lambda |
 ``root::logger`` | ``Usage: (logger)`` |

<span style="padding-left: 5px">logger struct
Initialize a logger object with a name and a log level that can be called
repeatedly to log to stdout. Supported log levels in order are :trace, :debug,
:info, :warn, :error, or :off. Calls to functions provided by struct are noops
unless that particular log level is enabled. To override the log level
specified in code set the environment variable SLSH_LOG_LEVEL to the desired
log level before initializing the struct.
Format is:
`{unix time} {pid} {log level}: [{logger-name}] {log-string}`
attribute: log-level private
attribute: log-level-int private
attribute: logger-name private
attribute: convert-log-level private
method: :get-log
method: :log-it
method: :trace
method: :debug
method: :info
method: :warn
method: :error
method: :init
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defn test-logger (log-name log-level)<br>
(let ((a-logger ((logger) :init log-name log-level))<br>
(str-list (list)))<br>
(append-to! str-list (list (a-logger :get-log :trace (str "test log " log-level))))<br>
(append-to! str-list (list (a-logger :get-log :debug (str "test log " log-level))))<br>
(append-to! str-list (list (a-logger :get-log :info (str "test log " log-level))))<br>
(append-to! str-list (list (a-logger :get-log :warn (str "test log " log-level))))<br>
(append-to! str-list (list (a-logger :get-log :error (str "test log " log-level))))<br>
(collect (filter (fn (x) (not (falsey? x))) str-list))))<br>
(test::assert-equal 5 (length (test-logger "test-logger" :trace)))<br>
(test::assert-equal 4 (length (test-logger "test-logger" :debug)))<br>
(test::assert-equal 3 (length (test-logger "test-logger" :info)))<br>
(test::assert-equal 2 (length (test-logger "test-logger" :warn)))<br>
(test::assert-equal 1 (length (test-logger "test-logger" :error)))<br>
(test::assert-equal 0 (length (test-logger "test-logger" :off)))<br>
(test::assert-error-msg ((logger) :init "test-logger" :bad-log-level) "log level must be a symbol one of: :trace, :debug, :info, :warn, :error, or :off")<br>
(test::assert-error-msg ((logger) :init 'bad-log-name :error) "in-logger-name must be a string.")<br>
<br>
</code>
</details>
<br>
### <a id="math-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Math forms](#math-contents)



| <a id="root::%" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``%``](#root::%-contents) | Type: Function |
 ``root::%`` | ``Usage: (% int int)`` |

<span style="padding-left: 5px">Remainder from dividing first int by the second.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0 (% 50 10))<br>
(test::assert-equal 5 (% 55 10))<br>
(test::assert-equal 1 (% 1 2))<br>
(test::assert-error (%))<br>
(test::assert-error (% 1))<br>
(test::assert-error (% 1 2 3))<br>
(test::assert-error (% 1 2.0))<br>
<br>
</code>
</details>
<br>


| <a id="root::*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*``](#root::*-contents) | Type: Function |
 ``root::*`` | ``Usage: (* number*)`` |

<span style="padding-left: 5px">Multiply a sequence of numbers.  (*) will return 1.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 1 (*))<br>
(test::assert-equal 5 (* 5))<br>
(test::assert-equal 5 (* 1 5))<br>
(test::assert-equal 5.0 (* 1.0 5))<br>
(test::assert-equal 7.5 (* 1.5 5))<br>
(test::assert-equal 7.5 (* 1.5 5.0))<br>
(test::assert-equal 15 (* 3 5))<br>
(test::assert-equal 8 (* 1 2 4))<br>
(test::assert-equal 16 (* 2 2 4))<br>
(test::assert-equal 16.0 (* 2 2.0 4))<br>
(test::assert-equal 16.0 (* 2.0 2.0 4.0))<br>
(test::assert-equal 55.0000000001 (* 100 0.55))<br>
(test::assert-error (* 1 2 4 "5"))<br>
<br>
</code>
</details>
<br>


| <a id="math::*euler*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*euler*``](#math::*euler*-contents) | Type: Float |
 ``math::*euler*`` | ``Usage: (print *e*)`` |

<span style="padding-left: 5px">Float representing euler's number.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 2.718281828459045 *e*)<br>
<br>
</code>
</details>
<br>


| <a id="math::*pi*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*pi*``](#math::*pi*-contents) | Type: Float |
 ``math::*pi*`` | ``Usage: (print *pi*)`` |

<span style="padding-left: 5px">Float representing pi.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 3.141592653589793 *pi*)<br>
<br>
</code>
</details>
<br>


| <a id="root::+" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``+``](#root::+-contents) | Type: Function |
 ``root::+`` | ``Usage: (+ number*)`` |

<span style="padding-left: 5px">Add a sequence of numbers.  (+) will return 0.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0 (+))<br>
(test::assert-equal 5 (+ 5))<br>
(test::assert-equal 5 (+ (values 5)))<br>
(test::assert-equal 5 (+ (values 5 6)))<br>
(test::assert-equal 10 (+ 5 (values 5 6)))<br>
(test::assert-equal 5 (+ 5.0))<br>
(test::assert-equal 6 (+ 1 5))<br>
(test::assert-equal 6.5 (+ 1 5.5))<br>
(test::assert-equal 7 (+ 1 2 4))<br>
(test::assert-error (+ 1 2 4 "5"))<br>
<br>
</code>
</details>
<br>


| <a id="root::-" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``-``](#root::--contents) | Type: Function |
 ``root::-`` | ``Usage: (- number+)`` |

<span style="padding-left: 5px">Subtract a sequence of numbers.  Requires at least one number (negate if only one number).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-error (-))<br>
(test::assert-error (- 5 "2"))<br>
(test::assert-equal -5 (- 5))<br>
(test::assert-equal -5.0 (- 5.0))<br>
(test::assert-equal -4 (- 1 5))<br>
(test::assert-equal -4.5 (- 1 5.5))<br>
(test::assert-equal 4 (- 10 2 4))<br>
(test::assert-equal 4.9 (- 10.9 2 4))<br>
<br>
</code>
</details>
<br>


| <a id="root::/" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``/``](#root::/-contents) | Type: Function |
 ``root::/`` | ``Usage: (/ number+)`` |

<span style="padding-left: 5px">Divide a sequence of numbers.  Requires at least two numbers.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 5 (/ 50 10))<br>
(test::assert-equal 5 (/ 50.0 10.0))<br>
(test::assert-equal 0 (/ 1 5))<br>
(test::assert-equal .2 (/ 1.0 5))<br>
(test::assert-equal .2 (/ 1.0 5.0))<br>
(test::assert-equal 5.5 (/ 5.5 1))<br>
(test::assert-equal 2 (/ 16 2 4))<br>
(test::assert-equal 5 (/ 100 2 5 2))<br>
(test::assert-error (/))<br>
(test::assert-error (/ 1))<br>
(test::assert-error (/ 1 0))<br>
(test::assert-error (/ 10 5 0))<br>
(test::assert-error (/ 10 "5" 2))<br>
<br>
</code>
</details>
<br>


| <a id="math::2pow" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``2pow``](#math::2pow-contents) | Type: Function |
 ``math::2pow`` | ``Usage: (2pow base)`` |

<span style="padding-left: 5px">Raise 2 to power of argument.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 1024 (2pow 10))<br>
(test::assert-equal (2pow (* 10 2)) (pow (2pow 10) 2))<br>
<br>
</code>
</details>
<br>


| <a id="math::abs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``abs``](#math::abs-contents) | Type: Function |
 ``math::abs`` | ``Usage: (abs arg)`` |

<span style="padding-left: 5px">Returns absolute value of arg.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 2.0 (abs 2))<br>
(test::assert-equal 144 (abs -144))<br>
(test::assert-equal 4.53 (abs -4.53))<br>
<br>
</code>
</details>
<br>


| <a id="math::arccos" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``arccos``](#math::arccos-contents) | Type: Function |
 ``math::arccos`` | ``Usage: (arccos num)`` |

<span style="padding-left: 5px">Take arccos of argument
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0.01 (cos (arccos 0.01)))<br>
<br>
</code>
</details>
<br>


| <a id="math::arcsin" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``arcsin``](#math::arcsin-contents) | Type: Function |
 ``math::arcsin`` | ``Usage: (arcsin num)`` |

<span style="padding-left: 5px">Take arcsin of argument
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0.01 (sin (arcsin 0.01)))<br>
<br>
</code>
</details>
<br>


| <a id="math::arctan" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``arctan``](#math::arctan-contents) | Type: Function |
 ``math::arctan`` | ``Usage: (arctan num)`` |

<span style="padding-left: 5px">Take arctan of argument
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0.01 (tan (arctan 0.01)))<br>
<br>
</code>
</details>
<br>


| <a id="math::ceil" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ceil``](#math::ceil-contents) | Type: Function |
 ``math::ceil`` | ``Usage: (ceil value)`` |

<span style="padding-left: 5px">Returns smallest integer greater than or equal to value.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 2.0 (ceil 2))<br>
(test::assert-equal 145 (ceil 144.444444))<br>
(test::assert-equal 5 (ceil 4.53))<br>
<br>
</code>
</details>
<br>


| <a id="math::cos" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cos``](#math::cos-contents) | Type: Function |
 ``math::cos`` | ``Usage: (cos num)`` |

<span style="padding-left: 5px">Take cos of argument
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal -0.14550003380861354 (cos 8))<br>
(test::assert-equal (cos 6) (/ (sin 6) (tan 6)))<br>
<br>
</code>
</details>
<br>


| <a id="math::exp" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``exp``](#math::exp-contents) | Type: Function |
 ``math::exp`` | ``Usage: (exp num)`` |

<span style="padding-left: 5px">Returns e ^ num, the exponential function.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal *euler* (exp 1))<br>
(test::assert-equal 1 (exp 0))<br>
(test::assert-equal 42 (exp (lne 42)))<br>
<br>
</code>
</details>
<br>


| <a id="math::floor" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``floor``](#math::floor-contents) | Type: Function |
 ``math::floor`` | ``Usage: (floor value)`` |

<span style="padding-left: 5px">Returns largest integer less than or equal to value.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 2.0 (floor 2))<br>
(test::assert-equal 144 (floor 144.444444))<br>
(test::assert-equal 4 (floor 4.53))<br>
<br>
</code>
</details>
<br>


| <a id="math::fract" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fract``](#math::fract-contents) | Type: Function |
 ``math::fract`` | ``Usage: (fract num)`` |

<span style="padding-left: 5px">Returns fractional part of a number
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0.9893582466233818 (fract 1911.9893582466233818))<br>
(test::assert-equal 0.0 (fract 1911))<br>
<br>
</code>
</details>
<br>


| <a id="math::lne" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``lne``](#math::lne-contents) | Type: Function |
 ``math::lne`` | ``Usage: (lne num)`` |

<span style="padding-left: 5px">Returns natural logarithm of number
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(def x 7.0)<br>
(def y 11.0)<br>
(test::assert-equal 1 (lne *euler*))<br>
(test::assert-equal 0 (lne 1))<br>
(test::assert-equal (lne (* x y)) (+ (lne x) (lne y)))<br>
(test::assert-equal (lne (/ x y)) (- (lne x) (lne y)))<br>
(test::assert-equal (lne (pow x y)) (* y (lne x)))<br>
<br>
</code>
</details>
<br>


| <a id="math::log" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``log``](#math::log-contents) | Type: Function |
 ``math::log`` | ``Usage: (log num base)`` |

<span style="padding-left: 5px">Returns log of number given base.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 8 (log 256 2))<br>
(test::assert-equal 3 (log 27 3))<br>
(test::assert-equal (log (pow 8 2) 10) (* 2 (log 8 10)))<br>
(test::assert-equal 1 (log 11 11))<br>
(test::assert-equal '-inf (log 0 11))<br>
(test::assert-equal (log 11 5) (/ (log 11 3) (log 5 3)))<br>
<br>
</code>
</details>
<br>


| <a id="math::log2" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``log2``](#math::log2-contents) | Type: Function |
 ``math::log2`` | ``Usage: (log2 num)`` |

<span style="padding-left: 5px">Returns log base 2 of input.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 7 (log2 128))<br>
(test::assert-equal (log 7 2) (/ 1.0 (log 2 7)))<br>
<br>
</code>
</details>
<br>


| <a id="math::pow" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pow``](#math::pow-contents) | Type: Function |
 ``math::pow`` | ``Usage: (pow base power)`` |

<span style="padding-left: 5px">Raise first argument to power of second argument.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 16 (pow 4 2))<br>
(test::assert-equal 10 (log (pow 2 10) 2))<br>
(test::assert-equal (pow 8 15) (* (pow 8 10) (pow 8 5)))<br>
(test::assert-equal (pow 100 3) (/ (pow 100 5) (pow 100 2)))<br>
(test::assert-equal 1 (pow 85 0))<br>
<br>
</code>
</details>
<br>


| <a id="math::round" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``round``](#math::round-contents) | Type: Function |
 ``math::round`` | ``Usage: (round arg)`` |

<span style="padding-left: 5px">Round arg to nearest int value.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 2.0 (round 2))<br>
(test::assert-equal 144 (round 144.444444))<br>
(test::assert-equal 5 (round 4.53))<br>
<br>
</code>
</details>
<br>


| <a id="math::sin" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sin``](#math::sin-contents) | Type: Function |
 ``math::sin`` | ``Usage: (sin num)`` |

<span style="padding-left: 5px">Take sin of argument
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0.9893582466233818 (sin 8))<br>
(test::assert-equal (sin 6) (* (tan 6) (cos 6)))<br>
<br>
</code>
</details>
<br>


| <a id="math::sqrt" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sqrt``](#math::sqrt-contents) | Type: Function |
 ``math::sqrt`` | ``Usage: (sqrt num)`` |

<span style="padding-left: 5px">Take square root of argument.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 2.0 (sqrt 4))<br>
(test::assert-equal 2.04939015319192 (sqrt 4.2))<br>
(test::assert-equal 12 (sqrt 144))<br>
<br>
</code>
</details>
<br>


| <a id="math::tan" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``tan``](#math::tan-contents) | Type: Function |
 ``math::tan`` | ``Usage: (tan num)`` |

<span style="padding-left: 5px">Take tan of argument
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal -6.799711455220379 (tan 8))<br>
(test::assert-equal (tan 6) (/ (sin 6) (cos 6)))<br>
<br>
</code>
</details>
<br>


| <a id="math::to-degrees" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``to-degrees``](#math::to-degrees-contents) | Type: Function |
 ``math::to-degrees`` | ``Usage: (to-degrees num)`` |

<span style="padding-left: 5px">Convert degrees to radians.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0 (- (to-degrees *pi*) 180))<br>
<br>
</code>
</details>
<br>


| <a id="math::to-radians" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``to-radians``](#math::to-radians-contents) | Type: Function |
 ``math::to-radians`` | ``Usage: (to-radians num)`` |

<span style="padding-left: 5px">Convert degrees to radians.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'math)<br>
(test::assert-equal 0 (- *pi* (to-radians 180)))<br>
<br>
</code>
</details>
<br>
### <a id="namespace-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Namespace forms](#namespace-contents)



| <a id="root::ns-auto-export" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-auto-export``](#root::ns-auto-export-contents) | Type: Macro |
 ``root::ns-auto-export`` | ``Usage: (ns-auto-export symbol)`` |

<span style="padding-left: 5px">Macro that takes a symbol, the symbol of the current namespace, and writes an
ns-export statement that includes all symbols defined in the namespaces scope
that do not begin with the '-' symbol. This is a convenience method that allows
user to avoid enumerating all symbols while also introducing a mechanism to
exclude symbols from being excluded. Note, if using ns-auto-export, it is
not possible to export a symbol that is already defined in another namespace,
if said functionality is desired the symbol must be manually exported with
another ns-export statement; ns-auto-export can be used in conjunction with
ns-export.
</span>
<br>
<br>


| <a id="root::ns-create" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-create``](#root::ns-create-contents) | Type: Function |
 ``root::ns-create`` | ``Usage: (ns-create namespace)`` |

<span style="padding-left: 5px">Creates and enters a new a namespace (must evaluate to a string or symbol).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-push 'test-ns-create)<br>
(def test-ns-enter *ns*)<br>
(ns-create 'ns-create-test-namespace)<br>
(def test-symbol "testing")<br>
(test::assert-equal "testing" test-symbol)<br>
(ns-enter test-ns-create::test-ns-enter)<br>
(test::assert-false (def? test-symbol))<br>
(ns-pop)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::ns-enter" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-enter``](#root::ns-enter-contents) | Type: Function |
 ``root::ns-enter`` | ``Usage: (ns-enter namespace)`` |

<span style="padding-left: 5px">Enters an existing namespace (must evaluate to a string or symbol).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-push 'test-ns-enter)<br>
(def test-ns-enter *ns*)<br>
(ns-create 'ns-enter-test-namespace)<br>
(def test-symbol "testing")<br>
(test::assert-equal "testing" test-symbol)<br>
(ns-enter test-ns-enter::test-ns-enter)<br>
(test::assert-false (def? test-symbol))<br>
(ns-enter 'ns-enter-test-namespace)<br>
(test::assert-true (def? test-symbol))<br>
(test::assert-equal "testing" test-symbol)<br>
(ns-enter test-ns-enter::test-ns-enter)<br>
(ns-pop)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::ns-exists?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-exists?``](#root::ns-exists?-contents) | Type: Function |
 ``root::ns-exists?`` | ``Usage: (ns-exists? namespace)`` |

<span style="padding-left: 5px">True if the supplied namespace exists (must evaluate to a string or symbol).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (ns-exists? 'ns-exists-test-namespace))<br>
(ns-push 'ns-exists-test-namespace)<br>
(ns-pop)<br>
(test::assert-true (ns-exists? 'ns-exists-test-namespace))<br>
<br>
</code>
</details>
<br>


| <a id="root::ns-export" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-export``](#root::ns-export-contents) | Type: Macro |
 ``root::ns-export`` | ``Usage: (ns-export symbol_or_sequence)`` |

<span style="padding-left: 5px">Export a symbol or list of symbols to be imported into other namespaces.
</span>
<br>
<br>


| <a id="root::ns-import" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-import``](#root::ns-import-contents) | Type: Macro |
 ``root::ns-import`` | ``Usage: (ns-import namespace)`` |

<span style="padding-left: 5px">Import any symbols exported from namespace into the current namespace.
</span>
<br>
<br>


| <a id="root::ns-list" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-list``](#root::ns-list-contents) | Type: Function |
 ``root::ns-list`` | ``Usage: (ns-list)`` |

<span style="padding-left: 5px">Returns a vector of all namespaces.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-not-includes "ns-list-test-namespace" (ns-list))<br>
(ns-push 'ns-list-test-namespace)<br>
(ns-pop)<br>
(test::assert-includes "ns-list-test-namespace" (ns-list))<br>
<br>
</code>
</details>
<br>


| <a id="root::ns-pop" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-pop``](#root::ns-pop-contents) | Type: Macro |
 ``root::ns-pop`` | ``Usage: (ns-pop)`` |

<span style="padding-left: 5px">Returns to the previous namespace saved in the last ns-push.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-ns-pop *ns*)<br>
(ns-push 'ns-pop-test-namespace)<br>
(test::assert-equal "ns-pop-test-namespace" *active-ns*)<br>
(ns-pop)<br>
(test::assert-equal test-ns-pop *ns*)<br>
<br>
</code>
</details>
<br>


| <a id="root::ns-push" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-push``](#root::ns-push-contents) | Type: Macro |
 ``root::ns-push`` | ``Usage: (ns-push 'namespace)`` |

<span style="padding-left: 5px">Pushes the current namespace on a stack for ns-pop and enters or creates namespace.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-ns-push *ns*)<br>
(ns-push 'ns-pop-test-namespace)<br>
; *ns* will not be right...<br>
(test::assert-equal "ns-pop-test-namespace" *active-ns*)<br>
(ns-push 'ns-pop-test-namespace2)<br>
(test::assert-equal "ns-pop-test-namespace2" *active-ns*)<br>
(ns-pop)<br>
(test::assert-equal "ns-pop-test-namespace" *active-ns*)<br>
(ns-pop)<br>
(test::assert-equal test-ns-push *ns*)<br>
<br>
</code>
</details>
<br>


| <a id="root::ns-symbols" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``ns-symbols``](#root::ns-symbols-contents) | Type: Function |
 ``root::ns-symbols`` | ``Usage: (ns-symbols namespace)`` |

<span style="padding-left: 5px">Returns the list of all symbols in namespace (must evaluate to a string or symbol).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-includes 'loop (ns-symbols 'root))<br>
(test::assert-not-includes 'dumb-symbol-xxx (ns-symbols 'root))<br>
(test::assert-includes 'car (ns-symbols 'root))<br>
<br>
</code>
</details>
<br>
### <a id="pair-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Pair forms](#pair-contents)
Operations on the 'Pair' type (aka Cons Cell) that can be used to create
traditional Lisp list structures. These are the default list structure and
are produced with bare parentheses in code. These lists can also be created by
building them up with joins or with the list form.


| <a id="root::car" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``car``](#root::car-contents) | Type: Function |
 ``root::car`` | ``Usage: (car pair)`` |

<span style="padding-left: 5px">Return the car (first item) from a pair.  If used on a proper list this will be the first element.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-pairs-two (list 'x 'y 'z))<br>
(test::assert-equal 'x (car tst-pairs-two))<br>
(test::assert-equal 10 (car '(10)))<br>
(test::assert-equal 9 (car '(9 11 13)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cdr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdr``](#root::cdr-contents) | Type: Function |
 ``root::cdr`` | ``Usage: (cdr pair)`` |

<span style="padding-left: 5px">Return the cdr (second item) from a pair.  If used on a proper list this will be the list minus the first element.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-pairs-three (list 'x 'y 'z))<br>
(test::assert-equal '(y z) (cdr tst-pairs-three))<br>
(test::assert-equal nil (cdr '(10)))<br>
(test::assert-equal '(13) (cdr '(9 13)))<br>
(test::assert-equal '(11 13) (cdr '(9 11 13)))<br>
<br>
</code>
</details>
<br>


| <a id="root::join" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``join``](#root::join-contents) | Type: Function |
 ``root::join`` | ``Usage: (join car cdr)`` |

<span style="padding-left: 5px">Create a pair with the provided car and cdr.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-pair-one (join 1 2))<br>
(test::assert-equal 1 (car tst-pair-one))<br>
(test::assert-equal 2 (cdr tst-pair-one))<br>
(test::assert-equal '(1 2 3) (join 1 (join 2 (join 3 nil))))<br>
<br>
</code>
</details>
<br>


| <a id="root::list" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``list``](#root::list-contents) | Type: Function |
 ``root::list`` | ``Usage: (list item0 item1 .. itemN)`` |

<span style="padding-left: 5px">Create a proper list from pairs with items 0 - N.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(1 2 3) (list 1 2 3))<br>
<br>
</code>
</details>
<br>


| <a id="root::xar!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``xar!``](#root::xar!-contents) | Type: Function |
 ``root::xar!`` | ``Usage: (xar! pair expression)`` |

<span style="padding-left: 5px">Destructive form that replaces the car (first item) in a pair with a new expression.
If used on a proper list will replace the first item.  Can be used on nil to
create a pair (expression . nil).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-pairs-three (list 'x 'y 'z))<br>
(test::assert-equal '(x y z) tst-pairs-three)<br>
(test::assert-equal '(s y z) (xar! tst-pairs-three 's))<br>
(test::assert-equal '(s y z) tst-pairs-three)<br>
(def tst-pairs-four nil)<br>
(test::assert-equal '() tst-pairs-four)<br>
(test::assert-equal '(t) (xar! tst-pairs-four 't))<br>
(test::assert-equal '(t) tst-pairs-four)<br>
<br>
</code>
</details>
<br>


| <a id="root::xdr!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``xdr!``](#root::xdr!-contents) | Type: Function |
 ``root::xdr!`` | ``Usage: (xdr! pair expression)`` |

<span style="padding-left: 5px">Destructive form that replaces the cdr (second item) in a pair with a new expression.
If used on a proper list will replace everthing after the first item.
Can be used on nil to create a pair (nil . expression).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def tst-pairs-five (list 'a 'b 'c))<br>
(test::assert-equal '(a b c) tst-pairs-five)<br>
(test::assert-equal '(a y z) (xdr! tst-pairs-five '(y z)))<br>
(test::assert-equal '(a y z) tst-pairs-five)<br>
(def tst-pairs-six nil)<br>
(test::assert-equal '() tst-pairs-six)<br>
(test::assert-equal '(nil . v) (xdr! tst-pairs-six 'v))<br>
(test::assert-equal '(nil . v) tst-pairs-six)<br>
<br>
</code>
</details>
<br>
### <a id="pair-ext-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Pair-ext forms](#pair-ext-contents)



| <a id="root::caaar" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caaar``](#root::caaar-contents) | Type: Lambda |
 ``root::caaar`` | ``Usage: (caaar lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 1 (caaar '(((1 4) 5) (6 3) 2)))<br>
<br>
</code>
</details>
<br>


| <a id="root::caadr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caadr``](#root::caadr-contents) | Type: Lambda |
 ``root::caadr`` | ``Usage: (caadr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 6 (caadr '((1 4 5) (6 3) 2)))<br>
<br>
</code>
</details>
<br>


| <a id="root::caar" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caar``](#root::caar-contents) | Type: Lambda |
 ``root::caar`` | ``Usage: (caar lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 1 (caar '((1) 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cadar" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cadar``](#root::cadar-contents) | Type: Lambda |
 ``root::cadar`` | ``Usage: (cadar lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 4 (cadar '((1 4 5) (6 3) 2)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cadddr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cadddr``](#root::cadddr-contents) | Type: Lambda |
 ``root::cadddr`` | ``Usage: (cadddr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 6 (cadddr '((1 7 8) (4 5) 2 6 (3 9))))<br>
<br>
</code>
</details>
<br>


| <a id="root::caddr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``caddr``](#root::caddr-contents) | Type: Lambda |
 ``root::caddr`` | ``Usage: (caddr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 6 (caddr '((1 4 5) 2 6)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cadr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cadr``](#root::cadr-contents) | Type: Lambda |
 ``root::cadr`` | ``Usage: (cadr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 2 (cadr '(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cdaar" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdaar``](#root::cdaar-contents) | Type: Lambda |
 ``root::cdaar`` | ``Usage: (cdaar lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(7 8) (cdaar '(((1 7 8) 4 5) 2 (6 3))))<br>
<br>
</code>
</details>
<br>


| <a id="root::cdadr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdadr``](#root::cdadr-contents) | Type: Lambda |
 ``root::cdadr`` | ``Usage: (cdadr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(9) (cdadr '(((1 7 8) 4 5) (2 9) (6 3))))<br>
<br>
</code>
</details>
<br>


| <a id="root::cdar" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdar``](#root::cdar-contents) | Type: Lambda |
 ``root::cdar`` | ``Usage: (cdar lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(4 5) (cdar '((1 4 5) 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cddar" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cddar``](#root::cddar-contents) | Type: Lambda |
 ``root::cddar`` | ``Usage: (cddar lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(5) (cddar '(((1 7 8) 4 5) 2 (6 3))))<br>
<br>
</code>
</details>
<br>


| <a id="root::cdddr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cdddr``](#root::cdddr-contents) | Type: Lambda |
 ``root::cdddr`` | ``Usage: (cdddr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(3 9) (cdddr '(((1 7 8) 4 5) 2 6 3 9)))<br>
<br>
</code>
</details>
<br>


| <a id="root::cddr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``cddr``](#root::cddr-contents) | Type: Lambda |
 ``root::cddr`` | ``Usage: (cddr lst)`` |

<span style="padding-left: 5px">Shorthand for car/cdr calls (a is car, d is cdr)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(3) (cddr '((1 4 5) 2 3)))<br>
<br>
</code>
</details>
<br>
### <a id="random-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Random forms](#random-contents)



| <a id="root::probool" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``probool``](#root::probool-contents) | Type: Function |
 ``root::probool`` | ``Usage: (probool), (probool numerator denominator)`` |

<span style="padding-left: 5px">PRObability of a BOOLean.
If no arguments are given, returns #t 1/2 of the time, otherwise takes two
integers, numerator and denominator, and returns #t numerator/denominator of the
time. Throws an error if denominator is 0. If (>= (/ numerator denominator) 1)
probool always returns true. If numerator is 0 probool always returns false.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def val0 (probool))<br>
(test::assert-true (or (= #t val0) (= nil val0)))<br>
(def val1 (probool 17 42))<br>
(test::assert-true (or (= #t val1) (= nil val1)))<br>
(test::assert-true (probool 1 1))<br>
(test::assert-false (probool 0 42))<br>
(test::assert-error-msg (probool 0 0) "Denominator can not be zero")<br>
(test::assert-error-msg (probool 0 0 0) "Expected zero or two numbers")<br>
<br>
</code>
</details>
<br>


| <a id="root::random" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``random``](#root::random-contents) | Type: Function |
 ``root::random`` | ``Usage: (random), (random limit)`` |

<span style="padding-left: 5px">Returns non-negative number less than limit and of the same type as limit.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def rand-int (random 100))<br>
(test::assert-true (and (> rand-int 0) (< rand-int 100))<br>
(def rand-float (random 1.0))<br>
(test::assert-true (and (> rand-float 0) (< rand-float 1)))<br>
(test::assert-error-msg (random -1) "Expected positive integer")<br>
(test::assert-error-msg (random 1 2) "Expected zero or one integers")<br>
<br>
</code>
</details>
<br>


| <a id="root::random-str" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``random-str``](#root::random-str-contents) | Type: Function |
 ``root::random-str`` | ``Usage: (random-str str-length [char-set])`` |

<span style="padding-left: 5px">Takes a positive integer, str-length, and one of :hex, :ascii, :alnum, or
a string. Returns random string of provided str-length composed of second argument:
:hex results in random hex string, :ascii results in random string of all printable
ascii characters, :alnum results in random string of all alphanumeric characters,
and providing a string results in a random string composed by sampling input.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-error-msg (random-str) "random-str: Missing required argument, see (doc 'random-str) for usage.")<br>
(test::assert-error-msg (random-str -1) "Expected positive number")<br>
(test::assert-error-msg (random-str 10) "random-str: Missing required argument, see (doc 'random-str) for usage.")<br>
(test::assert-equal 100 (length (random-str 10 :hex))<br>
(test::assert-true (str-contains "⚙" (random-str 42 "⚙"))<br>
(test::assert-equal 19 (length (random-str 19 :ascii)<br>
(test::assert-equal 91 (length (random-str 91 :alnum)<br>
<br>
</code>
</details>
<br>
### <a id="root-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Root forms](#root-contents)



| <a id="root::*read-table*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*read-table*``](#root::*read-table*-contents) | Type: HashMap |
 ``root::*read-table*`` | ``Usage: (print *read-table*)`` |

<span style="padding-left: 5px">Symbol that contains the current read table.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(hash-set! *read-table* #\$ 'shell-read::shell-read)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*read-table-terminal*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*read-table-terminal*``](#root::*read-table-terminal*-contents) | Type: HashMap |
 ``root::*read-table-terminal*`` | ``Usage: (print *read-table-terminal*)`` |

<span style="padding-left: 5px">Symbol that contains the current terminal read table.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(hash-set! *read-table-terminal* #\] 'nop-read)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::*string-read-table*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*string-read-table*``](#root::*string-read-table*-contents) | Type: HashMap |
 ``root::*string-read-table*`` | ``Usage: (print *string-read-table*)`` |

<span style="padding-left: 5px">Symbol that contains the current string read table.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(hash-set! *string-read-table* #\$ 'shell-read::shell-read)<br>
#t<br>
<br>
</code>
</details>
<br>
### <a id="scripting-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Scripting forms](#scripting-contents)



| <a id="root::*load-path*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*load-path*``](#root::*load-path*-contents) | Type: Vector |
 ``root::*load-path*`` | ``Usage: (set '*load-path* '("/path/one" "/path/two"))`` |

<span style="padding-left: 5px">Set the a list of paths to search for loading scripts with the load form.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(set '*load-path '("/path"))<br>
;(load "script-in-path")<br>
t<br>
<br>
</code>
</details>
<br>


| <a id="root::load" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``load``](#root::load-contents) | Type: Function |
 ``root::load`` | ``Usage: (load path) -> [last form value]`` |

<span style="padding-left: 5px">Read and eval a file (from path- a string).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-load-one nil)<br>
(def test-load-two nil)<br>
(def test-load-fn (open "/tmp/slsh-test-load.testing" :create :truncate))<br>
(write-line test-load-fn "(set! test-load-one \"LOAD TEST\") '(1 2 3)")<br>
(close test-load-fn)<br>
(set! test-load-two (load "/tmp/slsh-test-load.testing"))<br>
(test::assert-equal "LOAD TEST" test-load-one)<br>
(test::assert-equal '(1 2 3) test-load-two)<br>
<br>
</code>
</details>
<br>


| <a id="shell::mkli" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``mkli``](#shell::mkli-contents) | Type: Lambda |
 ``shell::mkli`` | ``Usage: (mkli filepath [namespace] [body])`` |

<span style="padding-left: 5px">"make lisp".creates a sl-sh shell script. given a file, a namespace (optional 2nd arg), and a string
to populate as the body (optional 3rd arg), make a canonincal blank sl-sh script
complete with all the relevant imports, and boilerplate namespace code taken
care of to speed up development.
It is recommended all calls to load are done at the top of the file (before
the calls to ns-enter or ns-create, in case a library sl-sh script calls a
library sl-sh script that created a namespace and forgot to call ns-pop.
This ensures the exported symbols for the first library's scripts
namespace are importable in the executing script's namespace.
All calls to ns-import happen after a ns is created and entered so the
current namespace is the namespace that houses all the imported symbols.
ns-export must be called before ns-pop so the appropriate symbols are
associated namespace, the one in which the symbols were created.
</span>
<br>
<br>
### <a id="sequence-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Sequence forms](#sequence-contents)
These macros will work on either a vector or a pair made into a proper list
(cons list).  Use these in preference to the vector/list specific versions when
possible (i.e. first vs car).
NOTE: list on this table can be a vector or a list.


| <a id="root::butlast" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``butlast``](#root::butlast-contents) | Type: Lambda |
 ``root::butlast`` | ``Usage: (butlast obj)`` |

<span style="padding-left: 5px">Produces the provided list minus the last element.  Nil if the list is empty or one element.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(1 2) (butlast '(1 2 3)))<br>
(assert-equal '(1 2) (butlast '#(1 2 3)))<br>
(assert-equal nil (butlast '(1)))<br>
(assert-equal nil (butlast '#(1)))<br>
(assert-equal nil (butlast '()))<br>
(assert-equal nil (butlast nil))<br>
(assert-equal nil (butlast '#()))<br>
<br>
</code>
</details>
<br>


| <a id="root::collect-copy" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``collect-copy``](#root::collect-copy-contents) | Type: Lambda |
 ``root::collect-copy`` | ``Usage: (collect-copy seq)`` |

<span style="padding-left: 5px">Produces a copy of the provided list (copy has same type as the parameter).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-colcl '(1 2 3))<br>
(assert-true (list? test-colcl))<br>
(def test-colcl2 (collect-copy test-colcl))<br>
(assert-true (list? test-colcl2))<br>
(assert-equal test-colcl test-colcl2)<br>
(def test-colcv '#(1 2 3))<br>
(assert-true (vec? test-colcv))<br>
(def test-colcv2 (collect-copy test-colcv))<br>
(assert-true (vec? test-colcv2))<br>
(assert-equal test-colcv test-colcv2)<br>
<br>
</code>
</details>
<br>


| <a id="root::empty-seq?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``empty-seq?``](#root::empty-seq?-contents) | Type: Lambda |
 ``root::empty-seq?`` | ``Usage: (empty-seq? obj) -> t/nil`` |

<span style="padding-left: 5px">`empty-seq?` returns true if a list or vector is empty and false/nil
otherwise. If a non list or non vector is passed in it returns nil.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (empty-seq? '(1 2 3)))<br>
(test::assert-false (empty-seq? '#(1 2 3)))<br>
(test::assert-true (empty-seq? '()))<br>
(test::assert-true (empty-seq? '#()))<br>
(test::assert-false (empty-seq? "aaa"))<br>
(test::assert-false (empty-seq? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::first" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``first``](#root::first-contents) | Type: Lambda |
 ``root::first`` | ``Usage: (first obj)`` |

<span style="padding-left: 5px">Produces the first element of the provided list or vector.  Nil if the
list/vector is nil/empty.  Note this is like car that works for lists and
vectors.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 1 (first '(1 2 3)))<br>
(assert-equal 1 (first '#(1 2 3)))<br>
(assert-equal nil (first '()))<br>
(assert-equal nil (first nil))<br>
(assert-equal nil (first '#()))<br>
<br>
</code>
</details>
<br>


| <a id="root::in?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``in?``](#root::in?-contents) | Type: Lambda |
 ``root::in?`` | ``Usage: (in? seq-to-search item-to-match)`` |

<span style="padding-left: 5px">Takes a [seq?](#root::seq?) that is not an [empty-seq?](#root::empty-seq?) and
returns true if the second argument is is in list, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(let ((vowels-list (list 'a 'e 'i 'o 'u)))<br>
(assert-true (in? vowels-list 'u))<br>
(assert-false (in? vowels-list 'c))<br>
(assert-true (in? (list (list)) (list)))<br>
(assert-false (in? 8 18)))<br>
<br>
</code>
</details>
<br>


| <a id="root::last" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``last``](#root::last-contents) | Type: Lambda |
 ``root::last`` | ``Usage: (last obj)`` |

<span style="padding-left: 5px">Produces the last element in a list or vector.  Nil if the list/vector is empty.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal 3 (last '(1 2 3)))<br>
(assert-equal 3 (last '#(1 2 3)))<br>
(assert-equal nil (last '()))<br>
(assert-equal nil (last nil))<br>
(assert-equal nil (last '#()))<br>
<br>
</code>
</details>
<br>


| <a id="root::non-empty-seq?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``non-empty-seq?``](#root::non-empty-seq?-contents) | Type: Lambda |
 ``root::non-empty-seq?`` | ``Usage: (non-empty-seq? obj) -> t/nil`` |

<span style="padding-left: 5px">`non-empty-seq?` returns true if a list or vector is not empty and false/nil
otherwise. If a non list or non vector is passed in it returns nil.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (non-empty-seq? '(1 2 3)))<br>
(test::assert-true (non-empty-seq? '#(1 2 3)))<br>
(test::assert-false (non-empty-seq? '()))<br>
(test::assert-false (non-empty-seq? '#()))<br>
(test::assert-false (non-empty-seq? "aaa"))<br>
(test::assert-false (non-empty-seq? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::qsort" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``qsort``](#root::qsort-contents) | Type: Lambda |
 ``root::qsort`` | ``Usage: (qsort sequence comp-lambda?) -> [sorted vector]`` |

<span style="padding-left: 5px">Sort a sequence using the quick sort algorithm.  Returns a vector of the sorted sequence.
The comp-lambda argument is optional, if provided it should be a lambda or
builtin that takes two arguments and return t or nil (it is the compare
function for the sort).  Defaults to < if not provided.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(1 2 3) (qsort '(2 3 1)))<br>
(test::assert-equal '(1 2 3) (qsort '#(2 3 1)))<br>
(test::assert-equal '(3 2 1) (qsort '(2 3 1) >))<br>
(test::assert-equal '(3 2 1) (qsort '#(2 3 1) (fn (a b) (< b a))))<br>
(test::assert-equal '("aaa" "aab" "aba" "baa" "bab" "ccc")<br>
(qsort '("aaa" "aab" "aba" "baa" "bab" "ccc")))<br>
(test::assert-equal '("aaa" "aab" "aba" "baa" "bab" "ccc")<br>
(qsort '("ccc" "bab" "baa" "aba" "aab" "aaa")))<br>
(test::assert-equal '("aaa" "aab" "aba" "baa" "bab" "ccc")<br>
(qsort '("aba" "bab" "aab" "ccc" "baa" "aaa")))<br>
(test::assert-equal '("ccc" "bab" "baa" "aba" "aab" "aaa")<br>
(qsort '("aba" "bab" "aab" "ccc" "baa" "aaa") >))<br>
(test::assert-equal '("ccc" "bab" "baa" "aba" "aab" "aaa")<br>
(qsort '("aba" "bab" "aab" "ccc" "baa" "aaa") (fn (a b) (> a b))))<br>
(test::assert-equal '() (qsort '()))<br>
(test::assert-equal '() (qsort '#()))<br>
(test::assert-equal '#() (qsort '()))<br>
(test::assert-equal '#() (qsort '#()))<br>
<br>
</code>
</details>
<br>


| <a id="root::rest" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``rest``](#root::rest-contents) | Type: Lambda |
 ``root::rest`` | ``Usage: (rest obj)`` |

<span style="padding-left: 5px">Produces the provided list or vector minus the first element.  Nil if the
list/vector is nil/empty or one element.  Note this is like cdr that works for
lists and vectors.  This calls vec-slice to create a new vector when called with
a vector (i.e. is much more efficient with lists).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal '(2 3) (rest '(1 2 3)))<br>
(assert-equal '(2 3) (rest '#(1 2 3)))<br>
(assert-equal nil (rest '(1)))<br>
(assert-equal nil (rest '#(1)))<br>
(assert-equal nil (rest '()))<br>
(assert-equal nil (rest nil))<br>
(assert-equal nil (rest '#()))<br>
<br>
</code>
</details>
<br>


| <a id="root::seq-for" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``seq-for``](#root::seq-for-contents) | Type: Macro |
 ``root::seq-for`` | ``Usage: (seq-for bind in items body)`` |

<span style="padding-left: 5px">Loops over each element in a sequence.  Simple version that works with lists and
vectors, use iterator::for in general.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def i 0)<br>
(seq-for x in '(1 2 3 4 5 6) (set! i (+ 1 i)))<br>
(assert-equal 6 i)<br>
<br>
</code>
</details>
<br>


| <a id="root::seq?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``seq?``](#root::seq?-contents) | Type: Lambda |
 ``root::seq?`` | ``Usage: (seq? expression) -> t/nil`` |

<span style="padding-left: 5px">True if expression is a list or vector, nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (seq? '(1 2 3)))<br>
(test::assert-true (seq? '#(1 2 3)))<br>
(test::assert-true (seq? '()))<br>
(test::assert-true (seq? '#()))<br>
(test::assert-false (seq? "aaa"))<br>
(test::assert-false (seq? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::setnth!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``setnth!``](#root::setnth!-contents) | Type: Lambda |
 ``root::setnth!`` | ``Usage: (setnth! idx obj sequence)`` |

<span style="padding-left: 5px">Sets idx item in the vector or list to obj, produces nil or errors on invalid input.
This is destructive!  Because vectors support indexing and lists do not, this is
a much faster operation for a vector (uses [builtin](root::builtin?) [vec-set!](root::vec-set!)
on input of type vector).  Return the list or vector that was modified.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def vctr (vec 0 1 2 3))<br>
(def vec-copy (collect-copy vctr))<br>
(setnth! 0 -5 vctr)<br>
(setnth! 1 -400000 vctr)<br>
(setnth! 2 -402202 vctr)<br>
(setnth! 3 -30000 vctr)<br>
(assert-not-equal vec-copy vctr)<br>
(setnth! 0 -4 vctr)<br>
(setnth! 1 -3 vctr)<br>
(setnth! 2 -2 vctr)<br>
(setnth! 3 -1 vctr)<br>
(assert-equal (list -4 -3 -2 -1) vctr)<br>
(assert-equal '(1 5 3) (setnth! 1 5 '#(1 2 3)) " Vector check")<br>
(assert-error (setnth! 0 1 '#()))<br>
(assert-error (setnth! 0 1 (vec)))<br>
(def lst (list 0 1 2 3))<br>
(def list-copy (collect-copy lst))<br>
(setnth! 0 -4 lst)<br>
(setnth! 1 -3 lst)<br>
(setnth! 2 -2 lst)<br>
(setnth! 3 -1 lst)<br>
(assert-not-equal list-copy lst)<br>
(setnth! 0 -4 lst)<br>
(setnth! 1 -3 lst)<br>
(setnth! 2 -2 lst)<br>
(setnth! 3 -1 lst)<br>
(assert-equal (list -4 -3 -2 -1) lst)<br>
(assert-equal '(1 5 3) (setnth! 1 5 '(1 2 3)) " List check")<br>
(assert-error (setnth! 0 1 '()))<br>
(assert-error (setnth! 0 1 (list)))<br>
<br>
</code>
</details>
<br>
### <a id="shell-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Shell forms](#shell-contents)
Forms to do shell operations like file tests, pipes, redirects, etc.


| <a id="root::*stderr*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*stderr*``](#root::*stderr*-contents) | Type: File |
 ``root::*stderr*`` | ``Usage: (write-line *stderr*)`` |

<span style="padding-left: 5px">File that connects to standard error by default.
Can be used in place of a write file object in any form that takes one.  Used
as the default for eprint and eprintln.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Use a file for stderr for test.<br>
(dyn *stderr* (open "/tmp/sl-sh.stderr.test" :create :truncate) (do (write-line *stderr* "Test Error") (close *stderr*)))<br>
(test::assert-equal "Test Error<br>
" (read-line (open "/tmp/sl-sh.stderr.test" :read)))<br>
<br>
</code>
</details>
<br>


| <a id="root::*stdin*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*stdin*``](#root::*stdin*-contents) | Type: File |
 ``root::*stdin*`` | ``Usage: (read-line *stdin*)`` |

<span style="padding-left: 5px">File that connects to standard in by default.
Can be used in place of a read file object in any form that takes one.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def stdin-test (open "/tmp/sl-sh.stdin.test" :create :truncate))<br>
(write-line stdin-test "Test line")<br>
(close stdin-test)<br>
; Use a file for stdin for test.<br>
(dyn *stdin* (open "/tmp/sl-sh.stdin.test" :read) (do (test::assert-equal "Test line<br>
" (read-line *stdin*)) (close *stdin*)))<br>
<br>
</code>
</details>
<br>


| <a id="root::*stdout*" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``*stdout*``](#root::*stdout*-contents) | Type: File |
 ``root::*stdout*`` | ``Usage: (write-line *stdout*)`` |

<span style="padding-left: 5px">File that connects to standard out by default.
Can be used in place of a write file object in any form that takes one.  Used
as the default for print and println.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Use a file for stdout for test.<br>
(dyn *stdout* (open "/tmp/sl-sh.stdout.test" :create :truncate) (do (write-line *stdout* "Test out") (close *stdout*)))<br>
(test::assert-equal "Test out<br>
" (read-line (open "/tmp/sl-sh.stdout.test" :read)))<br>
<br>
</code>
</details>
<br>


| <a id="shell::alias" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``alias``](#shell::alias-contents) | Type: Macro |
 ``shell::alias`` | ``Usage: (alias name body) or (alias name docstring body)`` |

<span style="padding-left: 5px">Create an alias, intended to be used with executables not lisp code (use defn
for that).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(shell::alias xx-echo1 (echo arg1))<br>
(shell::alias xx-echo2 "Dummy doc string" (echo))<br>
(let ((file-name (str (temp-dir)"/alias-test.out"))<br>
(topen))<br>
(out> file-name (xx-echo1 "stdout redir one1"))<br>
(out>> file-name (xx-echo2 "stdout redir one2"))<br>
(set! topen (open file-name :read))<br>
(test::assert-equal "arg1 stdout redir one1\n" (read-line topen))<br>
(test::assert-equal "stdout redir one2\n" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen))<br>
<br>
</code>
</details>
<br>


| <a id="shell::alias?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``alias?``](#shell::alias?-contents) | Type: Lambda |
 ``shell::alias?`` | ``Usage: (alias? name)`` |

<span style="padding-left: 5px">Provides boolean value confirming or denying given alias' presence
in set of registered aliases.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def xx-echo-not-alias #t)<br>
(test::assert-false (alias? 'xx-echo-not-alias))<br>
(test::assert-false (alias? 'xx-echo))<br>
(shell::alias xx-echo (echo -en))<br>
(test::assert-true (alias? 'xx-echo))<br>
(unalias xx-echo)<br>
(test::assert-false (alias? 'xx-echo))<br>
<br>
</code>
</details>
<br>


| <a id="shell::bg-color-rgb" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``bg-color-rgb``](#shell::bg-color-rgb-contents) | Type: Lambda |
 ``shell::bg-color-rgb`` | ``Usage: (bg-color-rgb red-val green-val blue-val)`` |

<span style="padding-left: 5px">Set the background color to the desired rgb where each arg is an integer between 0 and 255 inclusive.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "[48;2;128;128;128m" (bg-color-rgb 128 128 128))<br>
(test::assert-equal "[48;2;255;255;255m" (bg-color-rgb 255 255 255))<br>
(test::assert-equal "[48;2;255;0;0m" (bg-color-rgb 255 0 0))<br>
(test::assert-equal "[48;2;0;255;0m" (bg-color-rgb 0 255 0))<br>
(test::assert-equal "[48;2;0;0;255m" (bg-color-rgb 0 0 255))<br>
(test::assert-equal "[48;2;0;0;0m" (bg-color-rgb 0 0 0))<br>
<br>
</code>
</details>
<br>


| <a id="shell::clear-dirs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``clear-dirs``](#shell::clear-dirs-contents) | Type: Lambda |
 ``shell::clear-dirs`` | ``Usage: (clear-dirs)`` |

<span style="padding-left: 5px">Clears the directory stack.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(clear-dirs)<br>
(def cur-test-path (get-env PWD))<br>
(test::assert-equal '() (get-dirs))<br>
(pushd "/tmp")<br>
(def cur-test-path2 (get-env PWD))<br>
(test::assert-equal `(,cur-test-path) (get-dirs))<br>
(pushd (str-trim cur-test-path))<br>
(test::assert-equal `(,cur-test-path ,cur-test-path2) (get-dirs))<br>
(clear-dirs)<br>
(test::assert-equal '() (get-dirs))<br>
<br>
</code>
</details>
<br>


| <a id="shell::dirs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``dirs``](#shell::dirs-contents) | Type: Lambda |
 ``shell::dirs`` | ``Usage: (dirs)`` |

<span style="padding-left: 5px">List the directory stack.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(clear-dirs)<br>
(def cur-test-path (get-env PWD))<br>
(dyn *stdout* (open "/tmp/sl-sh.dirs.test" :create :truncate) (dirs))<br>
(test::assert-equal nil (read-line (open "/tmp/sl-sh.dirs.test" :read)))<br>
(pushd "/tmp")<br>
(def cur-test-path2 (get-env PWD))<br>
(dyn *stdout* (open "/tmp/sl-sh.dirs.test" :create :truncate) (dirs))<br>
(test::assert-equal cur-test-path (str-trim (read-line (open "/tmp/sl-sh.dirs.test" :read))))<br>
(pushd (str-trim cur-test-path))<br>
(dyn *stdout* (open "/tmp/sl-sh.dirs.test" :create :truncate) (dirs))<br>
(def test-dirs-file (open "/tmp/sl-sh.dirs.test" :read))<br>
(test::assert-equal cur-test-path (str-trim (read-line test-dirs-file)))<br>
(test::assert-equal cur-test-path2 (str-trim (read-line test-dirs-file)))<br>
(close test-dirs-file)<br>
(popd)<br>
(dyn *stdout* (open "/tmp/sl-sh.dirs.test" :create :truncate) (dirs))<br>
(test::assert-equal cur-test-path (str-trim (read-line (open "/tmp/sl-sh.dirs.test" :read))))<br>
(popd)<br>
(dyn *stdout* (open "/tmp/sl-sh.dirs.test" :create :truncate) (dirs))<br>
(test::assert-equal nil (read-line (open "/tmp/sl-sh.dirs.test" :read)))<br>
<br>
</code>
</details>
<br>


| <a id="root::epoch" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``epoch``](#root::epoch-contents) | Type: Function |
 ``root::epoch`` | ``Usage: (epoch)`` |

<span style="padding-left: 5px">Prints system time in milliseconds.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(epoch)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="shell::err>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err>``](#shell::err>-contents) | Type: Macro |
 ``shell::err>`` | ``Usage: (err> file body)`` |

<span style="padding-left: 5px">Redirect stderr to file, truncate the file first.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(err> "/tmp/sl-sh.err>.test" (eprintln "stderr redir one"))<br>
(def topen (open "/tmp/sl-sh.err>.test" :read))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(err> "/tmp/sl-sh.err>.test" (eprintln "stderr redir two"))<br>
(def topen (open "/tmp/sl-sh.err>.test" :read))<br>
(test::assert-equal "stderr redir two<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::err>>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err>>``](#shell::err>>-contents) | Type: Macro |
 ``shell::err>>`` | ``Usage: (err>> file body)`` |

<span style="padding-left: 5px">Redirect stderr to file, append the output.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(err> "/tmp/sl-sh.err>>.test" (eprintln "stderr redir one"))<br>
(def topen (open "/tmp/sl-sh.err>>.test" :read))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(err>> "/tmp/sl-sh.err>>.test" (eprintln "stderr redir two"))<br>
(def topen (open "/tmp/sl-sh.err>>.test" :read))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir two<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::err>null" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``err>null``](#shell::err>null-contents) | Type: Macro |
 ``shell::err>null`` | ``Usage: (err>null body)`` |

<span style="padding-left: 5px">Redirect stderr to null (/dev/null equivelent).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(err> "/tmp/sl-sh.err>null.test" (do (eprintln "stderr redir one")(err>null (eprintln "stdnull redir one"))))<br>
(def topen (open "/tmp/sl-sh.err>null.test" :read))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::fc" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fc``](#shell::fc-contents) | Type: Lambda |
 ``shell::fc`` | ``Usage: (fc)`` |

<span style="padding-left: 5px">Put the contents of the last command into a temporary file
([temp-dir](shell::temp-dir)), and open the temporary file in the text editor,
If the editor returns with an error code of 0 the contents of the
temporary file are executed. `fc` can be used in succession and the contents of
the temporary file are saved to the sl-sh history.
</span>
<br>
<br>


| <a id="shell::fg-color-rgb" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fg-color-rgb``](#shell::fg-color-rgb-contents) | Type: Lambda |
 ``shell::fg-color-rgb`` | ``Usage: (fg-color-rgb red-val green-val blue-val)`` |

<span style="padding-left: 5px">Set the foreground color to the desired rgb where each arg is an integer between 0 and 255 inclusive.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "[38;2;128;128;128m" (fg-color-rgb 128 128 128))<br>
(test::assert-equal "[38;2;255;255;255m" (fg-color-rgb 255 255 255))<br>
(test::assert-equal "[38;2;255;0;0m" (fg-color-rgb 255 0 0))<br>
(test::assert-equal "[38;2;0;255;0m" (fg-color-rgb 0 255 0))<br>
(test::assert-equal "[38;2;0;0;255m" (fg-color-rgb 0 0 255))<br>
(test::assert-equal "[38;2;0;0;0m" (fg-color-rgb 0 0 0))<br>
<br>
</code>
</details>
<br>


| <a id="shell::get-dirs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-dirs``](#shell::get-dirs-contents) | Type: Lambda |
 ``shell::get-dirs`` | ``Usage: (get-dirs)`` |

<span style="padding-left: 5px">Return the vector of directories.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(clear-dirs)<br>
(def cur-test-path (get-env PWD))<br>
(test::assert-equal '() (get-dirs))<br>
(pushd "/tmp")<br>
(def cur-test-path2 (get-env PWD))<br>
(test::assert-equal `(,cur-test-path) (get-dirs))<br>
(pushd (str-trim cur-test-path))<br>
(test::assert-equal `(,cur-test-path ,cur-test-path2) (get-dirs))<br>
(popd)<br>
(test::assert-equal `(,cur-test-path) (get-dirs))<br>
(popd)<br>
(test::assert-equal '() (get-dirs))<br>
<br>
</code>
</details>
<br>


| <a id="shell::getopts" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``getopts``](#shell::getopts-contents) | Type: Lambda |
 ``shell::getopts`` | ``Usage: (getopts options-map args)`` |

<span style="padding-left: 5px">Getopts takes a hash map and a vector of args and returns a hash map with all
the values extracted from the args and bound to the corresponding keys in the
provided hash map. Support for automatically generating documentation is
available with helper function [getopts-help](#shell::getopts-help).
Take this example script:
`sample-getopts.lisp`
```
#!/usr/bin/env sl-sh
(println "Passing: " args " to getopts")
;; getopts is given a hash map with one key, :-m, that corresponds to the flag,
;; -m, that it configures.
(def *sample-getopts-bindings*
(getopts
(make-hash
(list (join
:-m
(make-hash '((:arity . 1)
(:default . 0)
(:type . :int?))))))
args))
(println "The binding for -m is: " (hash-get *sample-getopts-bindings* :-m) "of type " (type (hash-get *sample-getopts-bindings* :-m)))
```
Running the script with one argument to the -m flag yields:
```
/sample-getopts.lisp -m 7
Passing: #("-m" "7") to getopts
The binding for -m is 7 of type Int
```
The hash map for the key :-m showcases the configuration keys that exist for
each flag: arity, :default, and :type. :arity specifies that the -m flag will
take one argument. :default specifies the bindng for :-m should be 0 if the
-m flag is not seen in args. :type :int? specifies that the binding should
be of that type, in this case int?. Running the script again with no -m
flag yields:
```
/sample-getopts.lisp
Passing: #() to getopts
The binding for -m is 0 of type Int
```
Demonstrating the :default binding of 0 for the symbol :-m since the -m flag
was not provided as an argument to the script.
Configuration keys for flags:
- :arity (optional)
Defaults to 0. If the arity is 0 the returned binding will be #t or nil.
Can be any integer. The integer value for the :arity corresponds to the
number of arguments getopts will enforce for this flag, if the number of
arguments provided to the flag is not equal to the specified arity an error
is thrown.
- :default (optional)
Use this as a default if the given flag is not provided at execution time.
- :type (optional)
Specify a type for every provided argument for the given flag. Types can be
any of: ("[hash?](#root::hash?)" "[falsey?](#root::falsey?)" "[list?](#root::list?)" "[lambda?](#root::lambda?)" "[fs-exists?](#root::fs-exists?)" "[float?](#root::float?)" "[string?](#root::string?)" "[int?](#root::int?)" "[fs-file?](#root::fs-file?)" "[char?](#root::char?)" "[vec?](#root::vec?)" "[macro?](#root::macro?)" "[symbol?](#root::symbol?)" "[fs-dir?](#root::fs-dir?)")
Rules for flags:
- Flags can be single character: -m -n -c etc.
- Flags of a single single character with arity 0 can be adjacent without the
need for additional dashes: -mnc
- Multiple flags of a single character with arity 0 can precede a flag of a
single character with arity N as long as said character appears last: -mne "foo"
- Flags can be multi-character as long as they are preceded by two dashes: --multi-char-arg
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;See tests/getopts.lisp<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="shell::getopts-help" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``getopts-help``](#shell::getopts-help-contents) | Type: Lambda |
 ``shell::getopts-help`` | ``Usage: (getopts-help bindings)`` |

<span style="padding-left: 5px">Companion function to getopts, call this function in the docstring of
any function that relies on getopts. getopts-help takes as an argument
the same bindings hash map that getopts does, and returns the documentation
for those getopts settings. getopts-help optionally supports a :doc keyword
for each command key. The value of doc is included in the resultant doc string
getopts-help returns.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;;; The following represents a full example usage of getopts<br>
;;; with documentation generated in the doc string of the function that uses<br>
;;; getopts:<br>
(def getopts-bindings<br>
(make-hash<br>
(list<br>
(def getopts-help-bindings<br>
(make-hash<br>
(list (join<br>
:-m<br>
(make-hash '((:arity . 1)<br>
(:default . 0)<br>
(:type . :int?)<br>
(:doc "this is displayed as<br>
two indented lines."))))<br>
(join<br>
:-b<br>
(make-hash '((:doc "this opts doc for -b<br>
goes on two lines!"))))<br>
(join<br>
:-a<br>
(make-hash '((:arity . 1)<br>
(:doc "this doc is for -a."))))<br>
(join<br>
:-k<br>
(make-hash '((:arity . 1)<br>
(:type . :int?)))))))<br>
(test::assert-equal<br>
(str-split :whitespace (getopts-help getopts-help-bindings))<br>
(str-split :whitespace (str-push! (str) #	 "-a" #	 "arity          1" #	 "this doc is for -a." #	 "-b" #	 "arity          0" #	 "this opts doc for -b" #	 "goes on two lines!" #	 "-k" #	 "arity          1" #	 "required type  :int?" #	 "-m" #	 "arity          1" #	 "default value  0" #	 "required type  :int?" #	 "this is displayed as" #	 "two indented lines.")))<br>
<br>
</code>
</details>
<br>


| <a id="root::history-context" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-context``](#root::history-context-contents) | Type: Function |
 ``root::history-context`` | ``Usage: (history-context :context_id context-string) -> nil`` |

<span style="padding-left: 5px">Sets the history context for searches.  Usually the current path but can be any
string.  Pass nil to set it to nothing.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(history-context :repl "/home")<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::history-empty?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-empty?``](#root::history-empty?-contents) | Type: Function |
 ``root::history-empty?`` | ``Usage: (history-empty? :context_id) -> t/nil`` |

<span style="padding-left: 5px">Returns true if history for context_id is empty, nil otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(history-empty? :repl)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::history-length" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-length``](#root::history-length-contents) | Type: Function |
 ``root::history-length`` | ``Usage: (history-length :context_id) -> int`` |

<span style="padding-left: 5px">Returns the number of history items for the given context.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(history-length :repl)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::history-nth" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-nth``](#root::history-nth-contents) | Type: Function |
 ``root::history-nth`` | ``Usage: (history-nth :context_id nth) -> String`` |

<span style="padding-left: 5px">Returns the history at index nth (the newest is 0).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(history-nth :repl 0)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::history-push" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-push``](#root::history-push-contents) | Type: Function |
 ``root::history-push`` | ``Usage: (history-push :context_id string) -> nil/t`` |

<span style="padding-left: 5px">Pushes string onto the history for the prompt context :context_id.
Returns true on success or nil on failure.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(history-push :repl "Some command")<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::history-push-throwaway" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``history-push-throwaway``](#root::history-push-throwaway-contents) | Type: Function |
 ``root::history-push-throwaway`` | ``Usage: (history-push-throwaway :context_id string) -> nil/t`` |

<span style="padding-left: 5px">Pushes string onto the history for the prompt context :context_id.  A throwaway
item will will only persist until the next command is read (use it to allow
editing of failed commands without them going into history).
Returns true on success or nil on failure.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(history-push-throwaway :repl "Some broken command")<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="shell::let-env" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``let-env``](#shell::let-env-contents) | Type: Macro |
 ``shell::let-env`` | ``Usage: (let-env vals &rest let_body)`` |

<span style="padding-left: 5px">Like let but sets environment variables that are reset after the macro finishes.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "" $LET-ENV-TEST-VAR-NOT-HERE)<br>
(let-env ((LET-ENV-TEST-VAR-NOT-HERE "here"))<br>
(test::assert-equal "here" $LET-ENV-TEST-VAR-NOT-HERE))<br>
(test::assert-equal "" $LET-ENV-TEST-VAR-NOT-HERE)<br>
<br>
</code>
</details>
<br>


| <a id="shell::out-err>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out-err>``](#shell::out-err>-contents) | Type: Macro |
 ``shell::out-err>`` | ``Usage: (out-err> file body)`` |

<span style="padding-left: 5px">Redirect both stdout and stderr to the same file, truncate the file first.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(out-err> "/tmp/sl-sh.out-err>.test" (do (println "stdout redir one")(eprintln "stderr redir one")))<br>
(def topen (open "/tmp/sl-sh.out-err>.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(out-err> "/tmp/sl-sh.out-err>.test" (do (syscall 'echo "stdout echo redir one")(eprintln "stderr redir one")))<br>
(def topen (open "/tmp/sl-sh.out-err>.test" :read))<br>
(test::assert-equal "stdout echo redir one<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(out-err> "/tmp/sl-sh.out-err>.test" (do (println "stdout redir two")(eprintln "stderr redir two")))<br>
(def topen (open "/tmp/sl-sh.out-err>.test" :read))<br>
(test::assert-equal "stdout redir two<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir two<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::out-err>>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out-err>>``](#shell::out-err>>-contents) | Type: Macro |
 ``shell::out-err>>`` | ``Usage: (out-err>> file body)`` |

<span style="padding-left: 5px">Redirect both stdout and stderr to the same file, append the output.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(out-err> "/tmp/sl-sh.out-err>>.test" (do (println "stdout redir one")(eprintln "stderr redir one")))<br>
(def topen (open "/tmp/sl-sh.out-err>>.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(out-err>> "/tmp/sl-sh.out-err>>.test" (do (println "stdout redir two")(eprintln "stderr redir two")))<br>
(def topen (open "/tmp/sl-sh.out-err>>.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-equal "stdout redir two<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir two<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::out-err>null" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out-err>null``](#shell::out-err>null-contents) | Type: Macro |
 ``shell::out-err>null`` | ``Usage: (out-err>null body)`` |

<span style="padding-left: 5px">Redirect both stdout and stderr to null (/dev/null equivelent).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(out-err> "/tmp/sl-sh.out-err>null.test" (do<br>
(println "stdout redir one")<br>
(eprintln "stderr redir one")<br>
(out-err>null (do<br>
(println "stdnull redir one")<br>
(eprintln "stdnull redir one")))))<br>
(def topen (open "/tmp/sl-sh.out-err>null.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-equal "stderr redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::out>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out>``](#shell::out>-contents) | Type: Macro |
 ``shell::out>`` | ``Usage: (out> file body)`` |

<span style="padding-left: 5px">Redirect stdout to file, truncate the file first.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(out> "/tmp/sl-sh.out>.test" (syscall 'echo "stdout redir one"))<br>
(def topen (open "/tmp/sl-sh.out>.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(out> "/tmp/sl-sh.out>.test" (syscall 'echo "stdout redir two"))<br>
(def topen (open "/tmp/sl-sh.out>.test" :read))<br>
(test::assert-equal "stdout redir two<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::out>>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out>>``](#shell::out>>-contents) | Type: Macro |
 ``shell::out>>`` | ``Usage: (out>> file body)`` |

<span style="padding-left: 5px">Redirect stdout to file, append the output.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(out> "/tmp/sl-sh.out>>.test" (syscall 'echo "stdout redir one"))<br>
(def topen (open "/tmp/sl-sh.out>>.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(out>> "/tmp/sl-sh.out>>.test" (syscall 'echo "stdout redir two"))<br>
(def topen (open "/tmp/sl-sh.out>>.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-equal "stdout redir two<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::out>null" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``out>null``](#shell::out>null-contents) | Type: Macro |
 ``shell::out>null`` | ``Usage: (out>null body)`` |

<span style="padding-left: 5px">Redirect stdout to null (/dev/null equivelent).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(out> "/tmp/sl-sh.out>null.test" (do (println "stdout redir one")(out>null (println "stdnull redir one"))))<br>
(def topen (open "/tmp/sl-sh.out>null.test" :read))<br>
(test::assert-equal "stdout redir one<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
<br>
</code>
</details>
<br>


| <a id="shell::popd" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``popd``](#shell::popd-contents) | Type: Lambda |
 ``shell::popd`` | ``Usage: (popd)`` |

<span style="padding-left: 5px">Pop first directory from directory stack and change to it.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def cur-test-path (get-env PWD))<br>
(pushd "/tmp")<br>
(def cur-test-path2 (get-env PWD))<br>
(assert-equal cur-test-path2 (get-env PWD))<br>
(popd)<br>
(assert-equal cur-test-path (get-env PWD))<br>
<br>
</code>
</details>
<br>


| <a id="root::prompt" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``prompt``](#root::prompt-contents) | Type: Function |
 ``root::prompt`` | ``Usage: (prompt string) -> string`` |

<span style="padding-left: 5px">Starts an interactive prompt (like the repl prompt) with the supplied prompt and
returns the input string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(def input-string (prompt "prompt> "))<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="shell::pushd" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pushd``](#shell::pushd-contents) | Type: Lambda |
 ``shell::pushd`` | ``Usage: (pushd dir)`` |

<span style="padding-left: 5px">Push current directory on the directory stack and change to new directory.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def cur-test-path (get-env PWD))<br>
(pushd "/tmp")<br>
(def cur-test-path2 (get-env PWD))<br>
(assert-equal cur-test-path2 (get-env PWD))<br>
(popd)<br>
(assert-equal cur-test-path (get-env PWD))<br>
<br>
</code>
</details>
<br>


| <a id="shell::register-alias" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``register-alias``](#shell::register-alias-contents) | Type: Lambda |
 ``shell::register-alias`` | ``Usage: (register-alias name)`` |

<span style="padding-left: 5px">Registers an alias to the current scope. Useful if unregistering or
ability to know whether an alias has been registered is desirable.
</span>
<br>
<br>


| <a id="shell::set-dirs-max" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``set-dirs-max``](#shell::set-dirs-max-contents) | Type: Lambda |
 ``shell::set-dirs-max`` | ``Usage: (set-dirs-max max)`` |

<span style="padding-left: 5px">Sets the max number of directories to save in the stack.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(clear-dirs)<br>
(def cur-test-path (get-env PWD))<br>
(pushd "/tmp")<br>
(def cur-test-path2 (get-env PWD))<br>
(pushd (str-trim cur-test-path))<br>
(pushd "/tmp")<br>
(pushd (str-trim cur-test-path))<br>
(test::assert-equal `(,cur-test-path ,cur-test-path2 ,cur-test-path ,cur-test-path2) (get-dirs))<br>
(clear-dirs)<br>
(set-dirs-max 3)<br>
(pushd "/tmp")<br>
(pushd (str-trim cur-test-path))<br>
(pushd "/tmp")<br>
(pushd (str-trim cur-test-path))<br>
(test::assert-equal `(,cur-test-path2 ,cur-test-path ,cur-test-path2) (get-dirs))<br>
<br>
</code>
</details>
<br>


| <a id="shell::syntax-off" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``syntax-off``](#shell::syntax-off-contents) | Type: Macro |
 ``shell::syntax-off`` | ``Usage: (syntax-off)`` |

<span style="padding-left: 5px">Turn off syntax highlighting at the repl.
</span>
<br>
<br>


| <a id="shell::syntax-on" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``syntax-on``](#shell::syntax-on-contents) | Type: Lambda |
 ``shell::syntax-on`` | ``Usage: (syntax-on)`` |

<span style="padding-left: 5px">Turn on syntax highlighting at the repl.
</span>
<br>
<br>


| <a id="shell::sys-command?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sys-command?``](#shell::sys-command?-contents) | Type: Lambda |
 ``shell::sys-command?`` | ``Usage: (sys-command? com)`` |

<span style="padding-left: 5px">True if the supplied command is a system command.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-true (sys-command? "ls"))<br>
(assert-false (sys-command? "rst-not-a-comand-strsnt"))<br>
<br>
</code>
</details>
<br>


| <a id="shell::temp-dir" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``temp-dir``](#shell::temp-dir-contents) | Type: Lambda |
 ``shell::temp-dir`` | ``Usage: (temp-dir)`` |

<span style="padding-left: 5px">Returns  environment variable if set, otherwise returns "/tmp".
</span>
<br>
<br>


| <a id="shell::timer" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``timer``](#shell::timer-contents) | Type: Lambda |
 ``shell::timer`` | ``Usage: (timer)`` |

<span style="padding-left: 5px">timer struct
Initialize a timer object that can be called repeatedly with :pr-next to
return relative time passed since instantiation and time since last called or
first instantiated.
attribute: start-time private
attribute: noop private
attribute: cnt private
attribute: prev-time private
attribute: curr-time private
attribute: timer-name private
method: :get-next
method: :pr-next
method: :init
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-timer ((timer) :init "test-timer" nil))<br>
(def timer-str (test-timer :get-next "event0"))<br>
(def timer-str-vec (str-split :whitespace timer-str))<br>
(test::assert-equal  "{0}[test-timer-event0]:" (vec-nth timer-str-vec 0))<br>
(test::assert-true (int? (str->int (vec-nth timer-str-vec 1))))<br>
(test::assert-true (int? (str->int (vec-nth timer-str-vec 4))))<br>
(def elapsed (vec-nth timer-str-vec 1))<br>
(def difference (vec-nth timer-str-vec 4))<br>
(test::assert-equal elapsed difference)<br>
<br>
</code>
</details>
<br>


| <a id="shell::unalias" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unalias``](#shell::unalias-contents) | Type: Macro |
 ``shell::unalias`` | ``Usage: (unalias name)`` |

<span style="padding-left: 5px">Remove an alias.  Use this instead of undef to make sure book keeping happens.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(shell::alias xx-echo (echo))<br>
(let ((file-name (str (temp-dir)"/unalias-test.out"))<br>
(topen))<br>
(out> file-name (xx-echo "stdout redir\none1"))<br>
(set! topen (open file-name :read))<br>
(test::assert-equal "stdout redir\n" (read-line topen))<br>
(test::assert-equal "one1\n" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen))<br>
(unalias xx-echo)<br>
(test::assert-error (xx-echo "error"))<br>
<br>
</code>
</details>
<br>


| <a id="shell::unregister-alias" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unregister-alias``](#shell::unregister-alias-contents) | Type: Lambda |
 ``shell::unregister-alias`` | ``Usage: (unregister-alias name)`` |

<span style="padding-left: 5px">Unregisters an alias, removing it from scope.
</span>
<br>
<br>


| <a id="root::version" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``version``](#root::version-contents) | Type: Function |
 ``root::version`` | ``Usage: (version)`` |

<span style="padding-left: 5px">Produce executable version as string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (string? (version)))<br>
<br>
</code>
</details>
<br>
### <a id="stats-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Stats forms](#stats-contents)



| <a id="stats::first-quartile" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``first-quartile``](#stats::first-quartile-contents) | Type: Function |
 ``stats::first-quartile`` | ``Usage: (first-quartile number+)`` |

<span style="padding-left: 5px">Returns first quartile of distribution of provided arguments.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (first-quartile) "expected at least one number")<br>
(test::assert-equal 2.5 (first-quartile 10 4 8 7 6 5 9 3 2 1))<br>
<br>
</code>
</details>
<br>


| <a id="stats::max" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``max``](#stats::max-contents) | Type: Function |
 ``stats::max`` | ``Usage: (max number+)`` |

<span style="padding-left: 5px">Returns maximum of provided arguments.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (max) "expected at least one number")<br>
(test::assert-equal 11 (max 10 4 8 7 6 5 9 3 2 1 11))<br>
<br>
</code>
</details>
<br>


| <a id="stats::mean" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``mean``](#stats::mean-contents) | Type: Function |
 ``stats::mean`` | ``Usage: (mean number+)`` |

<span style="padding-left: 5px">Average a sequence of numbers.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (mean) "expected at least one number")<br>
(test::assert-equal 5 (mean 5))<br>
(test::assert-equal 7.5 (mean 5 10))<br>
(test::assert-equal 5.5 (mean 1 2 3 4 5 6 7 8 9 10))<br>
<br>
</code>
</details>
<br>


| <a id="stats::median" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``median``](#stats::median-contents) | Type: Function |
 ``stats::median`` | ``Usage: (median number+)`` |

<span style="padding-left: 5px">Returns median of sequence of numbers.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (median) "expected at least one number")<br>
(test::assert-equal 5 (median 5))<br>
(test::assert-equal 7.5 (median 10 5))<br>
(test::assert-equal 5.5 (median 10 9 8 7 6 5 4 3 2 1))<br>
(test::assert-equal 6 (median 10 4 8 7 6 5 9 3 2 1 11))<br>
<br>
</code>
</details>
<br>


| <a id="stats::min" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``min``](#stats::min-contents) | Type: Function |
 ``stats::min`` | ``Usage: (min number+)`` |

<span style="padding-left: 5px">Returns minimum of provided arguments.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (min) "expected at least one number")<br>
(test::assert-equal 1 (min 10 4 8 7 6 5 9 3 2 1 11))<br>
<br>
</code>
</details>
<br>


| <a id="stats::mode" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``mode``](#stats::mode-contents) | Type: Function |
 ``stats::mode`` | ``Usage: (mode number+)`` |

<span style="padding-left: 5px">Returns mode of a sequence of numbers. Since distributions can be multimodal, mode returns a list.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (mode) "expected at least one number")<br>
(test::assert-equal (list 5) (mode 5))<br>
(test::assert-equal (list 1 3 4 5 6 7 8 9 10) (mode 1 3 4 5 6 7 8 9 10))<br>
(test::assert-equal (list 7.0) (mode 1 7.0 3 4 5 6 7 8 9 10))<br>
<br>
</code>
</details>
<br>


| <a id="stats::std-dev" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``std-dev``](#stats::std-dev-contents) | Type: Function |
 ``stats::std-dev`` | ``Usage: (std-dev number+)`` |

<span style="padding-left: 5px">Returns standard deviation of a sequence of numbers.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (std-dev) "expected at least one number")<br>
(test::assert-equal 3.0276503540974917 (std-dev 1 2 3 4 5 6 7 8 9 10))<br>
<br>
</code>
</details>
<br>


| <a id="stats::summary-stats" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``summary-stats``](#stats::summary-stats-contents) | Type: Function |
 ``stats::summary-stats`` | ``Usage: (summary-stats number+)`` |

<span style="padding-left: 5px">Returns hash map containing summary statistics and sorted array.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (summary-stats) "expected at least one number")<br>
(def distr (summary-stats 10 2 9 4 6 5 7 8 3 1))<br>
(test::assert-equal '#(1 2 3 4 5 6 7 8 9 10) (hash-get distr :vec))<br>
(test::assert-equal 5.5 (hash-get distr :med))<br>
(test::assert-equal 10 (hash-get distr :max))<br>
(test::assert-equal 3.0276503540974917 (hash-get distr :sd))<br>
(test::assert-equal 5.5 (hash-get distr :mean))<br>
(test::assert-equal '#(1 2 3 4 5 6 7 8 9 10) (hash-get distr :mode))<br>
(test::assert-equal 1 (hash-get distr :min))<br>
(test::assert-equal 8 (hash-get distr :q3))<br>
(test::assert-equal 2.5 (hash-get distr :q1))<br>
<br>
</code>
</details>
<br>


| <a id="stats::third-quartile" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``third-quartile``](#stats::third-quartile-contents) | Type: Function |
 ``stats::third-quartile`` | ``Usage: (third-quartile number+)`` |

<span style="padding-left: 5px">Returns third quartile of distribution of provided arguments.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-import 'stats)<br>
(test::assert-error-msg (third-quartile) "expected at least one number")<br>
(test::assert-equal 8 (third-quartile 10 4 8 7 6 5 9 3 2 1))<br>
<br>
</code>
</details>
<br>
### <a id="string-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[String forms](#string-contents)



| <a id="root::char->int" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char->int``](#root::char->int-contents) | Type: Function |
 ``root::char->int`` | ``Usage: (char->int a-char)`` |

<span style="padding-left: 5px">Reads a char or string, which may be composed of one or more unicode scalar values,
(see (doc 'codepoints) for more information) and returns a sum of the values.
This is not a hashing function, and only accepts one grapheme in the form of a
string or character. Graphemes composed of the same unicode scalar values will
result in the same integer value.'
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-error-msg (char->int (make-vec)) "char->int expects one argument of type Char or String")<br>
(test::assert-error-msg (char->int) "char->int: Missing required argument, see (doc 'char->int) for usage.")<br>
(test::assert-error-msg (char->int "a" "b") "char->int: Too many arguments, see (doc 'char->int) for usage.")<br>
(test::assert-error-msg (char->int "ab") "char->int takes one grapheme, multiple were provided.")<br>
(test::assert-error-msg (char->int "λ⚙") "char->int takes one grapheme, multiple were provided.")<br>
(test::assert-equal 97 (char->int "a"))<br>
(test::assert-equal 97 (char->int #\a))<br>
(test::assert-equal 7101 (char->int #\स्))<br>
(test::assert-equal 9881 (char->int (str "\" "u{2699}")))<br>
<br>
</code>
</details>
<br>


| <a id="root::codepoints" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``codepoints``](#root::codepoints-contents) | Type: Function |
 ``root::codepoints`` | ``Usage: (codepoints string)`` |

<span style="padding-left: 5px">Returns array of unicode scalars for each char in string. Note, a char
is not a grapheme. The hindi word namaste ("न" "म" "स्" "ते")
written in Devanagari script is 4 graphemes, but 6 unicode scalar values,
("u{928}" "u{92e}" "u{938}" "u{94d}" "u{924}" "u{947}");
[reference](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-error-msg (codepoints) "codepoints: Missing required argument, see (doc 'codepoints) for usage.")<br>
(test::assert-error-msg (codepoints (make-vec)) "codepoints expects one argument of type Char or String")<br>
(test::assert-error-msg (codepoints "a" "b") "codepoints: Too many arguments, see (doc 'codepoints) for usage.")<br>
(test::assert-equal (vec (str "\" "u{2699}")) (codepoints "⚙"))<br>
(test::assert-equal (vec (str "\" "u{938}") ((str "\" "u{94d}"))) (codepoints "स्"))<br>
(test::assert-equal (vec (str "\" "u{938}") ((str "\" "u{94d}"))) (codepoints #\स्))<br>
(test::assert-equal (vec (str "\" "u{61}")) (codepoints "a"))<br>
(test::assert-equal (vec (str "\" "u{61}")) (codepoints #\a))<br>
<br>
</code>
</details>
<br>


| <a id="root::do-unstr" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``do-unstr``](#root::do-unstr-contents) | Type: SpecialForm |
 ``root::do-unstr`` | ``Usage: (do-unstr arg0 ... argN) -> argN`` |

<span style="padding-left: 5px">Like do except if in an 'str' form then any processes will send their output to
*stdout* instead of being captured as a string ('undoes' a str for processes).
Note: does not convert arguments into strings, works like do.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; out> uses do-unstr so use a simplified one for these tests.<br>
(defmacro tst-out> (file body) `(dyn *stdout* (open ,file :create :truncate) ,body))<br>
(test::assert-equal "string 50 test<br>
" (str "string" " " 50 " " (syscall 'echo "test")))<br>
(test::assert-equal "string 50 test2<br>
" (str "string" " " 50 " " (tst-out> "/tmp/test-str-echo" (syscall 'echo "test2"))))<br>
(test::assert-equal "string 50 " (str "string" " " 50 " " (do-unstr (tst-out> "/tmp/test-str-echo" (syscall 'echo "test"))))<br>
<br>
</code>
</details>
<br>


| <a id="root::str" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str``](#root::str-contents) | Type: Function |
 ``root::str`` | ``Usage: (str arg0 ... argN) -> string`` |

<span style="padding-left: 5px">Make a new string with it's arguments.
Arguments will be turned into strings.  If an argument is a process then the
output of the process will be captured and put into the string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "stringsome" (str "string" "some"))<br>
(test::assert-equal "string" (str "string" ""))<br>
(test::assert-equal "string 50" (str "string" " " 50))<br>
(test::assert-equal "string 50 test<br>
" (str "string" " " 50 " " (syscall 'echo "test")))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-append" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-append``](#root::str-append-contents) | Type: Function |
 ``root::str-append`` | ``Usage: (str-append string string) -> string`` |

<span style="padding-left: 5px">Make a new string by appending two strings.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "stringsome" (str-append "string" "some"))<br>
(test::assert-equal "string" (str-append "string" ""))<br>
(test::assert-equal "string " (str-append "string" " "))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-bytes" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-bytes``](#root::str-bytes-contents) | Type: Function |
 ``root::str-bytes`` | ``Usage: (str-bytes string) -> int`` |

<span style="padding-left: 5px">Return number of bytes in a string (may be more then length).
Strings are utf8 so it chars and bytes may not be a one to one match.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 4 (str-bytes "Stau"))<br>
(test::assert-equal 0 (str-bytes ""))<br>
; Note 5 chars and 6 bytes because of the final char.<br>
(test::assert-equal 6 (str-bytes "StauΣ"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-cat-list" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-cat-list``](#root::str-cat-list-contents) | Type: Function |
 ``root::str-cat-list`` | ``Usage: (str-cat-list join-pattern sequence) -> string`` |

<span style="padding-left: 5px">Build a string by concatting a sequence with a join string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "stringxxxyyyxxxsome" (str-cat-list "xxx" '("string" "yyy" "some")))<br>
(test::assert-equal "string yyy some" (str-cat-list " " '("string" "yyy" "some")))<br>
(test::assert-equal "stringyyysome" (str-cat-list "" '("string" "yyy" "some")))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-clear!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-clear!``](#root::str-clear!-contents) | Type: Function |
 ``root::str-clear!`` | ``Usage: (str-clear! string) -> string`` |

<span style="padding-left: 5px">Clears a string.  This is a destructive form.
Returns the string it was given.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "" (str-clear! (str "string")))<br>
(def test-str-clear (str "def-string"))<br>
(test::assert-equal "" (str-clear! test-str-clear))<br>
(test::assert-equal "" test-str-clear)<br>
<br>
</code>
</details>
<br>


| <a id="root::str-contains" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-contains``](#root::str-contains-contents) | Type: Function |
 ``root::str-contains`` | ``Usage: (str-contains pattern string) -> t/nil`` |

<span style="padding-left: 5px">True if string contains pattern (both strings).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (str-contains "Stau" "Stausomething"))<br>
(test::assert-false (str-contains "StaU" "Stausomething"))<br>
(test::assert-true (str-contains "some" "Stausomething"))<br>
(test::assert-false (str-contains "Some" "Stausomething"))<br>
(test::assert-true (str-contains "thing" "Stausomething"))<br>
(test::assert-false (str-contains "Thing" "Stausomething"))<br>
(test::assert-true (str-contains "someΣ" "StausomeΣthing"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-empty?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-empty?``](#root::str-empty?-contents) | Type: Function |
 ``root::str-empty?`` | ``Usage: (str-empty? string) -> t/nil`` |

<span style="padding-left: 5px">Is a string empty?  Let's find out...
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (str-empty? ""))<br>
(test::assert-true (str-empty? (str-trim "   ")))<br>
(test::assert-false (str-empty? " "))<br>
(test::assert-false (str-empty? "string"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-iter-empty?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-empty?``](#root::str-iter-empty?-contents) | Type: Function |
 ``root::str-iter-empty?`` | ``Usage: (str-iter-empty? string) -> t/nil`` |

<span style="padding-left: 5px">Returns true if the iterator for the string is empty or finished.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter-start "test")<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-equal #\e (str-iter-next! test-iter-start))<br>
(test::assert-equal #\s (str-iter-next! test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
(def test-iter-start "test")<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-equal #\e (str-iter-next! test-iter-start))<br>
(str-push! test-iter-start "one")<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(str-clear! test-iter-start)<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-iter-next!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-next!``](#root::str-iter-next!-contents) | Type: Function |
 ``root::str-iter-next!`` | ``Usage: (str-iter-next! string) -> char`` |

<span style="padding-left: 5px">Returns the next char in the iterator for string.  Returns nil if iteration
is done.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter-start "y̆ΛλΣσ")<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(def test-iter-one (str-iter-next! test-iter-start))<br>
(test::assert-equal #\y̆ test-iter-one)<br>
(test::assert-true (= #\y̆ test-iter-one))<br>
(test::assert-false (= #\y test-iter-one))<br>
(test::assert-equal #\Λ (str-iter-next! test-iter-start))<br>
(test::assert-equal #\λ (str-iter-next! test-iter-start))<br>
(test::assert-equal #\Σ (str-iter-next! test-iter-start))<br>
(test::assert-equal #\σ (str-iter-next! test-iter-start))<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-iter-peek" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-peek``](#root::str-iter-peek-contents) | Type: Function |
 ``root::str-iter-peek`` | ``Usage: (str-iter-peek string) -> char`` |

<span style="padding-left: 5px">Returns the char that next will return in the iterator for string.  Returns nil if iteration
is done.  Does not advance the iterator.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter-start "y̆ΛλΣσ")<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(def test-iter-one (str-iter-next! test-iter-start))<br>
(test::assert-equal #\y̆ test-iter-one)<br>
(test::assert-true (= #\y̆ test-iter-one))<br>
(test::assert-false (= #\y test-iter-one))<br>
(test::assert-equal #\Λ (str-iter-peek test-iter-start))<br>
(test::assert-equal #\Λ (str-iter-next! test-iter-start))<br>
(test::assert-equal #\λ (str-iter-peek test-iter-start))<br>
(test::assert-equal #\λ (str-iter-next! test-iter-start))<br>
(test::assert-equal #\Σ (str-iter-peek test-iter-start))<br>
(test::assert-equal #\Σ (str-iter-next! test-iter-start))<br>
(test::assert-equal #\σ (str-iter-peek test-iter-start))<br>
(test::assert-equal #\σ (str-iter-next! test-iter-start))<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-iter-start" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-iter-start``](#root::str-iter-start-contents) | Type: Function |
 ``root::str-iter-start`` | ``Usage: (str-iter-start string) -> string`` |

<span style="padding-left: 5px">Starts or resets the iterator over a string.  Returns the input string with it's
iteration start created and at the first position.  Using the str-iter-* functions
is the proper way to get the chars of a string (since they are UTF a char is not
a fixed size so direct indexing is very inefficient).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-iter-start "test")<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-equal #\e (str-iter-next! test-iter-start))<br>
(test::assert-equal #\s (str-iter-next! test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-equal #\e (str-iter-next! test-iter-start))<br>
(str-iter-start test-iter-start)<br>
(test::assert-false (str-iter-empty? test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-equal #\e (str-iter-next! test-iter-start))<br>
(test::assert-equal #\s (str-iter-next! test-iter-start))<br>
(test::assert-equal #\t (str-iter-next! test-iter-start))<br>
(test::assert-true (str-iter-empty? test-iter-start))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-lower" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-lower``](#root::str-lower-contents) | Type: Function |
 ``root::str-lower`` | ``Usage: (str-lower string) -> string`` |

<span style="padding-left: 5px">Get all lower case string from a string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "stau" (str-lower "STAU"))<br>
(test::assert-equal "stau" (str-lower "stau"))<br>
(test::assert-equal "stau" (str-lower "Stau"))<br>
(test::assert-equal "stau" (str-lower "StaU"))<br>
(test::assert-equal "stau" (str-lower "sTaU"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-ltrim" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-ltrim``](#root::str-ltrim-contents) | Type: Function |
 ``root::str-ltrim`` | ``Usage: (str-ltrim string) -> string`` |

<span style="padding-left: 5px">Trim left whitspace from string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "some string" (str-ltrim "   some string"))<br>
(test::assert-equal "some string   " (str-ltrim "   some string   "))<br>
(test::assert-equal "some string   " (str-ltrim (str "   some string   ")))<br>
(test::assert-equal "some string   " (str-ltrim "some string   "))<br>
(test::assert-equal "some string" (str-ltrim "some string"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-map" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-map``](#root::str-map-contents) | Type: Function |
 ``root::str-map`` | ``Usage: (str-map lambda string) -> string`` |

<span style="padding-left: 5px">Make a new string by applying lambda to each char.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "XstringXstrX" (str-map (fn (ch) (if (= #\x ch) #\X ch)) "xstringxstrx"))<br>
(def test-str-map (str-map (fn (ch) (if (= #\x ch) #\X ch)) "xstringxstrx"))<br>
(test::assert-equal "XstringXstrX" test-str-map)<br>
(test::assert-true (string? test-str-map))<br>
(def test-str-map (str-map (fn (ch) (if (= #\x ch) #\X ch)) (str "xstringxstrx")))<br>
(test::assert-equal "XstringXstrX" test-str-map)<br>
(test::assert-true (string? test-str-map))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-nth" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-nth``](#root::str-nth-contents) | Type: Function |
 ``root::str-nth`` | ``Usage: (str-nth n string) -> char`` |

<span style="padding-left: 5px">Get the nth char of a string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal #\a (str-nth 2 "stau"))<br>
(test::assert-equal #\s (str-nth 0 "stau"))<br>
(test::assert-equal #\u (str-nth 3 "stau"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-push!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-push!``](#root::str-push!-contents) | Type: Function |
 ``root::str-push!`` | ``Usage: (str-push! string arg0 ... argN) -> string`` |

<span style="padding-left: 5px">Push the args (as strings) onto the string.  This is a destructive form.
Arguments will be turned into strings.  Returns the string it was given.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "stringsome" (str-push! (str "string") "some"))<br>
(def test-str-push (str "def-string"))<br>
(test::assert-equal "def-stringsome" (str-push! test-str-push "some"))<br>
(test::assert-equal "def-stringsome" test-str-push)<br>
<br>
</code>
</details>
<br>


| <a id="root::str-replace" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-replace``](#root::str-replace-contents) | Type: Function |
 ``root::str-replace`` | ``Usage: (str-replace string old-pattern new-pattern) -> string`` |

<span style="padding-left: 5px">Replace occurances of second string with third in the first string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "some yyy string" (str-replace "some xxx string" "xxx" "yyy"))<br>
(test::assert-equal "some yyy string yyy" (str-replace "some xxx string xxx" "xxx" "yyy"))<br>
(test::assert-equal "yyy some yyy string yyy" (str-replace "xxx some xxx string xxx" "xxx" "yyy"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-rsplit" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-rsplit``](#root::str-rsplit-contents) | Type: Function |
 ``root::str-rsplit`` | ``Usage: (str-rsplit split-pattern string) -> vector`` |

<span style="padding-left: 5px">Use a pattern to split a string into reverse order.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '("some" "yyy" "string") (str-rsplit "xxx" "stringxxxyyyxxxsome"))<br>
(test::assert-equal '("" "some" "yyy" "string") (str-rsplit "xxx" "stringxxxyyyxxxsomexxx"))<br>
(test::assert-equal '("some" "yyy" "string") (str-rsplit " " "string yyy some"))<br>
(test::assert-equal '("somexxxyyyxxxstring") (str-rsplit :whitespace "somexxxyyyxxxstring"))<br>
(test::assert-equal '("somexxxyyyxxxstring") (str-rsplit "zzz" "somexxxyyyxxxstring"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-rsplitn" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-rsplitn``](#root::str-rsplitn-contents) | Type: Function |
 ``root::str-rsplitn`` | ``Usage: (str-rsplitn n split-pattern string) -> vector`` |

<span style="padding-left: 5px">Use a pattern to split a string with at most n items returned in reverse order.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '("some" "yyy" "string") (str-rsplitn 3 "xxx" "stringxxxyyyxxxsome"))<br>
(test::assert-equal '("some" "yyy" "string") (str-rsplitn 4 "xxx" "stringxxxyyyxxxsome"))<br>
(test::assert-equal '("other" "string" "somexxxyyy") (str-rsplitn 3 "xxx" "somexxxyyyxxxstringxxxother"))<br>
(test::assert-equal '("somexxxyyyxxxstringxxxother") (str-rsplitn 1 "xxx" "somexxxyyyxxxstringxxxother"))<br>
(test::assert-equal '() (str-rsplitn 0 "xxx" "somexxxyyyxxxstringxxxzero"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-rtrim" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-rtrim``](#root::str-rtrim-contents) | Type: Function |
 ``root::str-rtrim`` | ``Usage: (str-rtrim string) -> string`` |

<span style="padding-left: 5px">Trim right whitespace from string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "   some string" (str-rtrim "   some string"))<br>
(test::assert-equal "   some string" (str-rtrim "   some string   "))<br>
(test::assert-equal "   some string" (str-rtrim (str "   some string   ")))<br>
(test::assert-equal "some string" (str-rtrim "some string   "))<br>
(test::assert-equal "some string" (str-rtrim "some string"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-split" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-split``](#root::str-split-contents) | Type: Function |
 ``root::str-split`` | ``Usage: (str-split split-pattern string) -> vector`` |

<span style="padding-left: 5px">Use a pattern to split a string (:whitespace to split on whitespace).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '("some" "yyy" "string") (str-split "xxx" "somexxxyyyxxxstring"))<br>
(test::assert-equal '("some" "yyy" "string" "") (str-split "xxx" "somexxxyyyxxxstringxxx"))<br>
(test::assert-equal '("" "some" "yyy" "string" "") (str-split "xxx" "xxxsomexxxyyyxxxstringxxx"))<br>
(test::assert-equal '("some" "yyy" "string") (str-split :whitespace "some yyy string"))<br>
(test::assert-equal '("somexxxyyyxxxstring") (str-split :whitespace "somexxxyyyxxxstring"))<br>
(test::assert-equal '("somexxxyyyxxxstring") (str-split "zzz" "somexxxyyyxxxstring"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-splitn" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-splitn``](#root::str-splitn-contents) | Type: Function |
 ``root::str-splitn`` | ``Usage: (str-splitn n split-pattern string) -> vector`` |

<span style="padding-left: 5px">Use a pattern to split a string with at most n items.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '("some" "yyy" "string") (str-splitn 3 "xxx" "somexxxyyyxxxstring"))<br>
(test::assert-equal '("some" "yyy" "string") (str-splitn 4 "xxx" "somexxxyyyxxxstring"))<br>
(test::assert-equal '("some" "yyy" "stringxxxother") (str-splitn 3 "xxx" "somexxxyyyxxxstringxxxother"))<br>
(test::assert-equal '("somexxxyyyxxxstringxxxother") (str-splitn 1 "xxx" "somexxxyyyxxxstringxxxother"))<br>
(test::assert-equal '() (str-splitn 0 "xxx" "somexxxyyyxxxstringxxxzero"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-starts-with" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-starts-with``](#root::str-starts-with-contents) | Type: Function |
 ``root::str-starts-with`` | ``Usage: (str-starts-with pattern string) -> t/nil`` |

<span style="padding-left: 5px">True if string start with pattern (both strings).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (str-starts-with "Stau" "Stausomething"))<br>
(test::assert-false (str-starts-with "StaU" "Stausomething"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-sub" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-sub``](#root::str-sub-contents) | Type: Function |
 ``root::str-sub`` | ``Usage: (str-sub string start [length]) -> string`` |

<span style="padding-left: 5px">Return a substring from a string given start (0 based) and optional length.
If length is 0 or not provided produces the rest of the string from start to
string end.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "string" (str-sub "stringxxxyyyxxxsome" 0 6))<br>
(test::assert-equal "some" (str-sub "stringxxxyyyxxxsome" 15 4))<br>
(test::assert-equal "yyy" (str-sub "stringxxxyyyxxxsome" 9 3))<br>
(test::assert-equal "some" (str-sub "stringxxxyyyxxxsome" 15))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-trim" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-trim``](#root::str-trim-contents) | Type: Function |
 ``root::str-trim`` | ``Usage: (str-trim string) -> string`` |

<span style="padding-left: 5px">Trim right and left whitespace from string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "some string" (str-trim "   some string"))<br>
(test::assert-equal "some string" (str-trim "   some string   "))<br>
(test::assert-equal "some string" (str-trim (str "   some string   ")))<br>
(test::assert-equal "some string" (str-trim "some string   "))<br>
(test::assert-equal "some string" (str-trim "some string"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str-upper" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str-upper``](#root::str-upper-contents) | Type: Function |
 ``root::str-upper`` | ``Usage: (str-upper string) -> string`` |

<span style="padding-left: 5px">Get all upper case string from a string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "STAU" (str-upper "STAU"))<br>
(test::assert-equal "STAU" (str-upper "stau"))<br>
(test::assert-equal "STAU" (str-upper "Stau"))<br>
(test::assert-equal "STAU" (str-upper "StaU"))<br>
(test::assert-equal "STAU" (str-upper "sTaU"))<br>
<br>
</code>
</details>
<br>


| <a id="root::with-padding" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``with-padding``](#root::with-padding-contents) | Type: Lambda |
 ``root::with-padding`` | ``Usage: (with-padding target padding padding-char &rest padding-keyword)`` |

<span style="padding-left: 5px">Given a "target" string, a keyword specifying the type of padding, an integer
length to pad to "padding", and a character to use for padding
"padding-char", return a string with length equal to the specified padding
length, only if the target string is less than the amount of padding specified.
The resultant string defaults to right padding and is composed of the target
string followed by "padding" number of "padding-char". If left or center
padding is desired pass one of the keywords :left or :center as the final
argument to the function. For completeness, the keyword :right is also
supported.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "test......" (with-padding "test" 10 "."))<br>
(test::assert-equal "test......" (with-padding "test" 10 "." :right))<br>
(test::assert-equal "......test" (with-padding "test" 10 "." :left))<br>
(test::assert-equal "...test..." (with-padding "test" 10 "." :center))<br>
(test::assert-equal "..tests..." (with-padding "tests" 10 "." :center))<br>
<br>
</code>
</details>
<br>
### <a id="struct-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Struct forms](#struct-contents)



| <a id="struct::defstruct" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``defstruct``](#struct::defstruct-contents) | Type: Macro |
 ``struct::defstruct`` | ``Usage: (defstruct name &rest fields)`` |

<span style="padding-left: 5px">Define a structure.  This produces a lambda with name that will create an instance.
Each 'field' will add an attribute, method or trait.
Use (attr-name doc-str? default? [:rw | :ro | :wo]?) if a final access modifier is not provided then it is private.
NOTE: for attributes, if the default value is a string then doc-str is not optional (but can be empty).
Use (:fn name doc-str? body) to add a method.
Use (:impl trait) to add a trait.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(struct::defstruct test-struct<br>
; fields<br>
(a "a private attribute" nil)<br>
(b "a read/write attribute" "bee" :rw)<br>
(c "a read only attribute" "see" :ro)<br>
(d "a write only attribute" "dee" :wo)<br>
; methods<br>
(:fn what-d (self) d)<br>
(:fn what-a (self) a))<br>
(def ts (test-struct))<br>
(assert-equal nil (ts :what-a))<br>
(assert-equal "dee" (ts :what-d))<br>
(ts :set-d "something else")<br>
(assert-equal "something else" (ts :what-d))<br>
(assert-equal "bee" (ts :b))<br>
(assert-equal "see" (ts :c))<br>
(ts :set-b "queen")<br>
(assert-equal "queen" (ts :b))<br>
<br>
</code>
</details>
<br>


| <a id="struct::deftrait" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``deftrait``](#struct::deftrait-contents) | Type: Macro |
 ``struct::deftrait`` | ``Usage: (deftrait name &rest fields)`` |

<span style="padding-left: 5px">Define a trait.  Traits define methods that are added to structures (and usually require one or
more methods in the struct.  Use them to provide common functionality shared by different structures.
Use (:fn name doc-str? body) to add a method.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(ns-push 'test-deftrait)<br>
(struct::deftrait test-trait<br>
; Require what-a and what-d in the structures that implement this trait.<br>
(:fn aaa (self) (self :what-a))<br>
(:fn ddd (self) (self :what-d)))<br>
(struct::defstruct test-struct<br>
; fields<br>
(a "a private attribute" nil)<br>
(b "a read/write attribute" "bee" :rw)<br>
(c "a read only attribute" "see" :ro)<br>
(d "a write only attribute" "dee" :wo)<br>
; methods<br>
(:fn what-d (self) d)<br>
(:fn what-a (self) a)<br>
(:impl test-deftrait::test-trait))<br>
(def ts (test-struct))<br>
(test::assert-equal nil (ts :aaa))<br>
(test::assert-equal "dee" (ts :ddd))<br>
(ts :set-d "something else")<br>
(test::assert-equal "something else" (ts :ddd))<br>
(test::assert-equal "bee" (ts :b))<br>
(test::assert-equal "see" (ts :c))<br>
(ts :set-b "queen")<br>
(test::assert-equal "queen" (ts :b))<br>
(ns-pop)<br>
<br>
</code>
</details>
<br>
### <a id="system-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[System forms](#system-contents)



| <a id="root::bg" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``bg``](#root::bg-contents) | Type: Function |
 ``root::bg`` | ``Usage: (bg job-id?)`` |

<span style="padding-left: 5px">Put a job in the background.
If no job id is specified use the last job.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(bg)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::exit" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``exit``](#root::exit-contents) | Type: Function |
 ``root::exit`` | ``Usage: (exit code?)`` |

<span style="padding-left: 5px">Exit shell with optional status code.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
; Exit is overridden in the test harness...<br>
;(test::assert-equal 10 (wait (fork (exit 10))))<br>
;(test::assert-equal 11 (wait (fork (exit 11))))<br>
;(test::assert-equal 12 (wait (fork (exit 12))))<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::export" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``export``](#root::export-contents) | Type: Function |
 ``root::export`` | ``Usage: (export symbol string) -> string`` |

<span style="padding-left: 5px">Export a key and value to the shell environment.  Second arg will be made a string and returned.
Key can not contain the '=' character.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "ONE" (export 'TEST_EXPORT_ONE "ONE"))<br>
(test::assert-equal "ONE" $TEST_EXPORT_ONE))<br>
(test::assert-equal "ONE1" (export 'TEST_EXPORT_ONE ONE1))<br>
(test::assert-equal "ONE1" $TEST_EXPORT_ONE))<br>
(test::assert-equal "TWO" (export "TEST_EXPORT_TWO" "TWO"))<br>
(test::assert-equal "TWO" $TEST_EXPORT_TWO))<br>
(test::assert-equal "THREE" $(export TEST_EXPORT_THREE THREE))<br>
(test::assert-equal "THREE" $TEST_EXPORT_THREE))<br>
(test::assert-error (export '=TEST_EXPORT_THREE "THREE"))<br>
(test::assert-error (export 'TEST=EXPORT_THREE "THREE"))<br>
(test::assert-error (export 'TEST_EXPORT_THREE= "THREE"))<br>
(test::assert-error $(export TEST_EXPORT_THREE= THREE))<br>
<br>
</code>
</details>
<br>


| <a id="root::fg" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fg``](#root::fg-contents) | Type: Function |
 ``root::fg`` | ``Usage: (fg job-id?)`` |

<span style="padding-left: 5px">Put a job in the foreground.
If no job id is specified use the last job.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(fg)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::fork" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``fork``](#root::fork-contents) | Type: SpecialForm |
 ``root::fork`` | ``Usage: (fork exp) -> process`` |

<span style="padding-left: 5px">Forks the provided expression in the background as a job and returns the process
object.  If the expression that is forked returns an integer (that fits an i32)
then it will become the exit code.  Calling exit explicitly will also set the
exit code.  Otherwise exit code is 0 for success and 1 for an error.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def fork-test (fork (+ (* 11 5) 2)))<br>
(test::assert-equal 57 (wait fork-test))<br>
(def fork-time (time (wait (fork (sleep 1000)))))<br>
(test::assert-true (> fork-time 1.0))<br>
<br>
</code>
</details>
<br>


| <a id="root::get-env" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-env``](#root::get-env-contents) | Type: SpecialForm |
 ``root::get-env`` | ``Usage: (get_env key) -> string`` |

<span style="padding-left: 5px">Lookup key in the system environment (env variable).  Returns an empty sting if key does not exist.
Note: key is not evaluated.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "ONE" (export 'TEST_EXPORT_ONE "ONE"))<br>
(test::assert-equal "ONE" $TEST_EXPORT_ONE))<br>
(test::assert-equal "ONE" (get-env TEST_EXPORT_ONE))<br>
(test::assert-equal "" (get-env TEST_EXPORT_ONE_NA))<br>
<br>
</code>
</details>
<br>


| <a id="root::get-pid" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``get-pid``](#root::get-pid-contents) | Type: Function |
 ``root::get-pid`` | ``Usage: (get-pid)`` |

<span style="padding-left: 5px">Return the pid of running process (shell).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (int? (get-pid)))<br>
<br>
</code>
</details>
<br>


| <a id="root::jobs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``jobs``](#root::jobs-contents) | Type: Function |
 ``root::jobs`` | ``Usage: (jobs)`` |

<span style="padding-left: 5px">Print list of jobs with ids.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(jobs)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::pid" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pid``](#root::pid-contents) | Type: Function |
 ``root::pid`` | ``Usage: (pid proc)`` |

<span style="padding-left: 5px">Return the pid of a process.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def pid-test (syscall 'echo "-n"))<br>
(test::assert-true (int? (pid pid-test)))<br>
(test::assert-true (int? (pid (fork ((fn () nil))))))<br>
(test::assert-error (pid 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::pipe" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pipe``](#root::pipe-contents) | Type: Function |
 ``root::pipe`` | ``Usage: (pipe [expression]+)`` |

<span style="padding-left: 5px">Setup a pipe between processes or expressions.  Pipe will take one or more
expressions, each one but the last will be forked into a new process with it's
stdin being the output of the last expression.  The first expression uses the
current stdin and the last expression outputs to the current stdout.  Pipe works
with system commands as well as sl-sh forms (lambdas, etc).  Note it connects
the stdin/stdout of processes so if used with a lambda it should read stdin to
get the previous output and write to stdout to pass to the next expression in
the pipe (i.e. pipe will not interact with parameters or anything else).
If pipe starts with :err then stderr will also be piped into the output,
ie (pipe :err (...)(...)...).
Pipes also support using a read file as the first expression (the file contents
become stdin for the next form) and a write file as the last expression
(previous output will be written to the file).  For instance pipe can be used
to copy a file with (pipe (open IN_FILE :read)(open OUT_FILE :create)), note
this example does not close the files.
Pipes can be nested including piping through a lambda that itself uses pipes.
Pipe will return a multiple values, the first/primary is the final form for the
pipe and the process objects for each part of the pipe are next (first element
can be found with (values-nth 1 return-val), etc).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def pipe-test (str (pipe (print "one<br>
two<br>
three")(syscall 'grep "two"))))<br>
(test::assert-equal "two<br>
" pipe-test)<br>
(def pipe-test (str (pipe (pipe (syscall 'echo "one<br>
two<br>
twotwo<br>
three")(syscall 'grep "two"))(syscall 'grep "twotwo"))))<br>
(test::assert-equal "twotwo<br>
" pipe-test)<br>
(def pipe-test-dir (str (temp-dir)"/tst-pipe-dir"))<br>
$(mkdir $pipe-test-dir)<br>
(def tsync (open "${pipe-test-dir}/test1" :create))<br>
(pipe (print "one<br>
two<br>
two2<br>
three") (syscall 'grep "two") tsync)<br>
(close tsync)<br>
(def topen (open "${pipe-test-dir}/test1" :read))<br>
(test::assert-equal "two<br>
" (read-line topen))<br>
(test::assert-equal "two2<br>
" (read-line topen))<br>
(test::assert-false (read-line topen))<br>
(close topen)<br>
(def topen (open "${pipe-test-dir}/test1" :read))<br>
(def pipe-test (str (pipe topen (syscall 'grep "two2"))))<br>
(close topen)<br>
(test::assert-equal "two2<br>
" pipe-test)<br>
$(rm "${pipe-test-dir}/test1")<br>
(let ((file-name "${pipe-test-dir}/pipe-err.test")<br>
(fin))<br>
(err> (open "${pipe-test-dir}/pipe-test.junk" :create) (do<br>
(pipe (eprintln "error")(do (print *stdin*)(println "normal"))(open file-name :create :truncate))<br>
(set! fin (open file-name :read))<br>
(test::assert-equal "normal\n" (read-line fin))<br>
(test::assert-equal nil (read-line fin))<br>
(close fin)<br>
(pipe :err (eprintln "error")(do (print *stdin*)(println "normal"))(open file-name :create :truncate))<br>
(set! fin (open file-name :read))<br>
(test::assert-equal "error\n" (read-line fin))<br>
(test::assert-equal "normal\n" (read-line fin))<br>
(close fin)))<br>
$(rm "${pipe-test-dir}/pipe-test.junk")<br>
$(rm $file-name)<br>
)<br>
$(rmdir $pipe-test-dir)<br>
<br>
</code>
</details>
<br>


| <a id="root::reap-jobs" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``reap-jobs``](#root::reap-jobs-contents) | Type: Function |
 ``root::reap-jobs`` | ``Usage: (reap-jobs) -> nil`` |

<span style="padding-left: 5px">Reaps any completed jobs.  Only intended to be used by code implemeting the REPL
loop or something similiar, this is probably not the form you are searching for.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
;(reap-jobs)<br>
#t<br>
<br>
</code>
</details>
<br>


| <a id="root::sleep" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sleep``](#root::sleep-contents) | Type: Function |
 ``root::sleep`` | ``Usage: (sleep milliseconds) -> nil`` |

<span style="padding-left: 5px">Sleep for the provided milliseconds (must be a positive integer).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-sleep-var (time (sleep 1000)))<br>
(assert-true (> test-sleep-var 1.0))<br>
<br>
</code>
</details>
<br>


| <a id="root::syscall" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``syscall``](#root::syscall-contents) | Type: Function |
 ``root::syscall`` | ``Usage: (syscall system-command arg0 ... argN)`` |

<span style="padding-left: 5px">Execute the provided system command with the supplied arguments.
System-command can evalute to a string or symbol.
The args (0..n) are evaluated.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-syscall-one (str (syscall "echo" "-n" "syscall-test")))<br>
(test::assert-equal "syscall-test" test-syscall-one)<br>
(def test-syscall-one (str (syscall 'echo "-n" "syscall-test2")))<br>
(test::assert-equal "syscall-test2" test-syscall-one)<br>
(def test-syscall-echo "echo")<br>
(def test-syscall-one (str (syscall test-syscall-echo "-n" "syscall-test3")))<br>
(test::assert-equal "syscall-test3" test-syscall-one)<br>
<br>
</code>
</details>
<br>


| <a id="root::time" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``time``](#root::time-contents) | Type: Function |
 ``root::time`` | ``Usage: (time form) -> eval-time`` |

<span style="padding-left: 5px">Evalutes the provided form and returns the seconds it ran for (as float with fractional part).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-sleep-var (time (sleep 1100)))<br>
(assert-true (> test-sleep-var 1.1))<br>
<br>
</code>
</details>
<br>


| <a id="root::unexport" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``unexport``](#root::unexport-contents) | Type: Function |
 ``root::unexport`` | ``Usage: (unexport symbol)`` |

<span style="padding-left: 5px">Remove a var from the current shell environment.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "ONE" (export 'TEST_EXPORT_ONE "ONE"))<br>
(test::assert-equal "ONE" $TEST_EXPORT_ONE))<br>
(unexport 'TEST_EXPORT_ONE)<br>
(test::assert-equal "" $TEST_EXPORT_ONE))<br>
<br>
</code>
</details>
<br>


| <a id="root::wait" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``wait``](#root::wait-contents) | Type: Function |
 ``root::wait`` | ``Usage: (wait proc-to-wait-for)`` |

<span style="padding-left: 5px">Wait for a process to end and return it's exit status.
Wait can be called multiple times if it is given a process
object (not just a numeric pid).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def wait-test (wait (err>null (syscall 'ls "/does/not/exist/123"))))<br>
(test::assert-true (> wait-test 0))<br>
(def wait-test2 (fork (* 11 5)))<br>
(test::assert-equal 55 (wait wait-test2))<br>
(test::assert-equal 55 (wait wait-test2))<br>
(test::assert-equal 55 (wait wait-test2))<br>
<br>
</code>
</details>
<br>
### <a id="test-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Test forms](#test-contents)



| <a id="test::assert-equal" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-equal``](#test::assert-equal-contents) | Type: Lambda |
 ``test::assert-equal`` | ``Usage: (assert-equal expected-val right-val &rest args)`` |

<span style="padding-left: 5px">Test expected-val and right-val for equality.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (test::assert-equal 1 1))<br>
(test::assert-false (test::assert-equal (list 1 2) (list 1)))<br>
(test::assert-true (test:: assert-equal (list 1 2) (list 1 2)))<br>
<br>
</code>
</details>
<br>


| <a id="test::assert-error" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-error``](#test::assert-error-contents) | Type: Macro |
 ``test::assert-error`` | ``Usage: (assert-error form)`` |

<span style="padding-left: 5px">Test asserts an error is thrown.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-error (err "error thrown"))<br>
<br>
</code>
</details>
<br>


| <a id="test::assert-error-msg" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-error-msg``](#test::assert-error-msg-contents) | Type: Macro |
 ``test::assert-error-msg`` | ``Usage: (assert-error-msg form msg)`` |

<span style="padding-left: 5px">Test asserts an error is thrown with a given message.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-error-msg (err "error thrown") "error thrown")<br>
<br>
</code>
</details>
<br>


| <a id="test::assert-false" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-false``](#test::assert-false-contents) | Type: Lambda |
 ``test::assert-false`` | ``Usage: (assert-false value &rest args)`` |

<span style="padding-left: 5px">Test for falsiness.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false #f)<br>
(test::assert-false nil)<br>
<br>
</code>
</details>
<br>


| <a id="test::assert-not-equal" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-not-equal``](#test::assert-not-equal-contents) | Type: Lambda |
 ``test::assert-not-equal`` | ``Usage: (assert-not-equal expected-val right-val &rest args)`` |

<span style="padding-left: 5px">Test expected-val and right-val are not equal.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (test::assert-not-equal 1 1))<br>
(test::assert-true (test::assert-not-equal 1 2))<br>
(test::assert-false (test::assert-not-equal (list 1 2 3) (list 1 2 3))<br>
<br>
</code>
</details>
<br>


| <a id="test::assert-true" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``assert-true``](#test::assert-true-contents) | Type: Lambda |
 ``test::assert-true`` | ``Usage: (assert-true value &rest args)`` |

<span style="padding-left: 5px">Test for truthiness.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true #t)<br>
<br>
</code>
</details>
<br>


| <a id="test::run-example" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``run-example``](#test::run-example-contents) | Type: Macro |
 ``test::run-example`` | ``Usage: (run-example sym)`` |

<span style="padding-left: 5px">Given a symbol, run any tests found in example section if exists.
</span>
<br>
<br>
### <a id="threading-macros-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Threading-macros forms](#threading-macros-contents)



| <a id="root::->" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``->``](#root::->-contents) | Type: Macro |
 ``root::->`` | ``Usage: (-> &rest args)`` |

<span style="padding-left: 5px">inserts result of previous expression as second argument to current expression.
First argument is not evaluated.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal<br>
(str "I go at the beginning.I'll be stuck in the middle.I'll be at the end.")<br>
(-> "I go at the beginning."<br>
(str "I'll be stuck in the middle.")<br>
(str "I'll be at the end.")))<br>
<br>
</code>
</details>
<br>


| <a id="root::->>" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``->>``](#root::->>-contents) | Type: Macro |
 ``root::->>`` | ``Usage: (->> &rest args)`` |

<span style="padding-left: 5px">inserts result of previous expression as last argument to current expression.
First argument is not evaluated.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(assert-equal<br>
(str "I'll be at the beginning.I'll be more in the middle.I go at the end.")<br>
(->> "I go at the end."<br>
(str "I'll be more in the middle.")<br>
(str "I'll be at the beginning.")))<br>
<br>
</code>
</details>
<br>


| <a id="root::chain" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``chain``](#root::chain-contents) | Type: Macro |
 ``root::chain`` | ``Usage: (chain init arg0 &rest args)`` |

<span style="padding-left: 5px">Inserts result of previous expression into place indicated by the _ symbol
in the next expression. This macro is useful when nested actions
need to occur but when a more sequential enumeration of steps is preferable.
To demonstrate a sequence of operations:
(fn (x) (delta (charlie (bravo (alpha x)))))
can be unwound, and more idiomatically stated with the chain macro:
(fn (x) (chain x (alpha _) (bravo _) (charlie _) (delta _)))
The flexibility of using the _ symbol as a placeholder means subsequent
results can be "threaded" into any desired place in a clause which is an
advantage over the thread first, [->](#root::->) and thread last,
[->>](#root::->>), macros
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "Ordered sentence: I will become a properly worded statement."<br>
(chain "a properly worded "<br>
(str "will " "become " _)<br>
(str "I " _ "statement.")<br>
(str "Ordered sentence: " _)))<br>
(test::assert-equal 3 (chain 7 (% _ 4)))<br>
(test::assert-equal 9 (chain 7 (% _ 4) (+ 1 8)))<br>
(test::assert-equal (list 6 12 18) (collect (chain 7<br>
(% _ 4)<br>
(range _ 10)<br>
(map (fn (x) (* x 2)) _)<br>
(filter (fn (x) (= 0 (% x 3))) _))))<br>
<br>
</code>
</details>
<br>


| <a id="root::chain-and" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``chain-and``](#root::chain-and-contents) | Type: Macro |
 ``root::chain-and`` | ``Usage: (chain-and init arg0 &rest args)`` |

<span style="padding-left: 5px">Evaluates each sexp, if true, inserts result of previous expression into place
indicated by the _ symbol. This macro is useful when nested actions
need to take place but the operations should stop and return nil if any step
evaluates to false.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (chain-and "howdy" (string? _) (= _ "howdy")))<br>
(test::assert-true  (chain-and "howdy" (str _ " partner") (= _ "howdy partner")))<br>
(defn formatted? (alleged-time-str)<br>
(let* ((do-fst (fn (tst start end)<br>
(fn (orig-string)<br>
(let ((result (get-error<br>
(chain orig-string<br>
(str-sub _ start (- end start))<br>
(str->int _)<br>
(tst _)))))<br>
(if (= :ok (car result))<br>
(if (cdr result) orig-string nil)<br>
nil)))))<br>
(valid-year? (do-fst (fn (x) (> x -1)) 0 4))<br>
(valid-month? (do-fst (fn (x) (and (> x 0) (< x 13))) 4 6))<br>
(valid-day? (do-fst (fn (x) (and (> x 0) (< x 32))) 6 8))<br>
(valid-hour? (do-fst (fn (x) (and (> x 0) (< x 24))) 9 11))<br>
(valid-minute? (do-fst (fn (x) (and (> x -1) (< x 60))) 11 13))<br>
(valid-second? (do-fst (fn (x) (and (> x -1) (< x 60))) 13 15))<br>
(is-zulu? (fn (orig-string) (= "Z" (str-sub orig-string 15 1)))))<br>
(chain-and alleged-time-str<br>
(valid-year? _)<br>
(valid-month? _)<br>
(valid-day? _)<br>
(valid-hour? _)<br>
(valid-minute? _)<br>
(valid-second? _)<br>
(is-zulu? _)<br>
#t)))<br>
(test::assert-true (formatted? "20210227T154705Z"))<br>
(test::assert-false (formatted? "20210227T154705P"))<br>
(test::assert-false (formatted? "2021022TT154705Z"))<br>
(test::assert-false (formatted? "20210237T154705Z"))<br>
(test::assert-false (formatted? "2021222TT154705Z"))<br>
(test::assert-false (formatted? "20210227T254705Z"))<br>
(test::assert-false (formatted? "20210227T158705Z"))<br>
(test::assert-false (formatted? "20210227T154795Z"))<br>
<br>
</code>
</details>
<br>


| <a id="root::chain-when" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``chain-when``](#root::chain-when-contents) | Type: Macro |
 ``root::chain-when`` | ``Usage: (chain-when init arg0 &rest args)`` |

<span style="padding-left: 5px">Tests the car of each arg0/args pair. If true the cdr of the arg0/args pair
is evaluated. The chaining component of this macro works by threading init
through the cdr of each pair whose car evaluates to true. Useful for building
or modifying an object based on a set of boolean conditions.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(defn add-number-attributes (n)<br>
(chain-when (make-hash)<br>
((not (= (% n 2) 0)) (hash-set! _ :property :odd))<br>
((= (% n 2) 0) (hash-set! _ :property :even))<br>
((= 0 n) (hash-set! _ :zero :zero))<br>
((= 42 n) (err "N CAN NOT BE 42"))<br>
((> n 0) (hash-set! _ :symmetry :positive))<br>
((< n 0) (hash-set! _ :symmetry :negative))))<br>
(test::assert-equal :odd (hash-get (add-number-attributes 3) :property))<br>
(test::assert-equal :even (hash-get (add-number-attributes 4) :property))<br>
(test::assert-false (hash-get (add-number-attributes 4) :zero))<br>
(test::assert-equal :positive (hash-get (add-number-attributes 4) :symmetry))<br>
(test::assert-equal :negative (hash-get (add-number-attributes -4) :symmetry))<br>
(test::assert-equal :zero (hash-get (add-number-attributes 0) :zero))<br>
(test::assert-error-msg (add-number-attributes 42) "N CAN NOT BE 42")<br>
<br>
</code>
</details>
<br>
### <a id="type-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Type forms](#type-contents)
These forms provide information/tests about an objects underlying type.


| <a id="root::boolean?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``boolean?``](#root::boolean?-contents) | Type: Function |
 ``root::boolean?`` | ``Usage: (boolean? expression)`` |

<span style="padding-left: 5px">True if the expression is true(#t) or false(#f), false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (boolean? #f))<br>
(test::assert-true (boolean? #t))<br>
(test::assert-false (boolean? nil))<br>
(test::assert-false (boolean? nil))<br>
(test::assert-false (boolean? 1))<br>
(test::assert-false (boolean? "str"))<br>
<br>
</code>
</details>
<br>


| <a id="root::builtin?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``builtin?``](#root::builtin?-contents) | Type: Function |
 ``root::builtin?`` | ``Usage: (builtin? expression)`` |

<span style="padding-left: 5px">True if the expression is a builtin function or special form, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (builtin? type))<br>
(test::assert-true (builtin? if))<br>
(test::assert-false (builtin? (fn () ())))<br>
(test::assert-false (builtin? caar))<br>
(test::assert-false (builtin? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::char?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``char?``](#root::char?-contents) | Type: Function |
 ``root::char?`` | ``Usage: (char? expression)`` |

<span style="padding-left: 5px">True if the expression is a char, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (char? #\a))<br>
(test::assert-false (char? 1))<br>
(test::assert-false (char? "a"))<br>
<br>
</code>
</details>
<br>


| <a id="root::false?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``false?``](#root::false?-contents) | Type: Function |
 ``root::false?`` | ``Usage: (false? expression)`` |

<span style="padding-left: 5px">True if the expression is false(#f) (false type NOT nil), false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (false? #f))<br>
(test::assert-false (false? nil))<br>
(test::assert-false (false? nil))<br>
(test::assert-false (false? 1))<br>
(test::assert-false (false? "str"))<br>
<br>
</code>
</details>
<br>


| <a id="root::falsey?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``falsey?``](#root::falsey?-contents) | Type: Function |
 ``root::falsey?`` | ``Usage: (falsey? under-test) -> bool`` |

<span style="padding-left: 5px">Returns true if the expression under-test evaluates to nil or false.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (falsey? nil))<br>
(test::assert-true (falsey? #f))<br>
(test::assert-false (falsey? #t))<br>
(test::assert-false (falsey? "false"))<br>
<br>
</code>
</details>
<br>


| <a id="root::file?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``file?``](#root::file?-contents) | Type: Function |
 ``root::file?`` | ``Usage: (file? expression)`` |

<span style="padding-left: 5px">True if the expression is a file, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (file? (open :stdout)))<br>
(test::assert-false (file? (fn () ())))<br>
(test::assert-false (file? caar))<br>
(test::assert-false (file? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::float->int" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``float->int``](#root::float->int-contents) | Type: Function |
 ``root::float->int`` | ``Usage: (float->int float) -> int`` |

<span style="padding-left: 5px">Cast a float as an int.  Truncates.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 0 (float->int 0.0))<br>
(test::assert-equal 10 (float->int 10.0))<br>
(test::assert-equal 10 (float->int 10.1))<br>
(test::assert-equal 10 (float->int 10.5))<br>
(test::assert-equal 10 (float->int 10.9))<br>
(test::assert-equal -101 (float->int -101.99))<br>
(test::assert-error (float->int "not int"))<br>
<br>
</code>
</details>
<br>


| <a id="root::float?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``float?``](#root::float?-contents) | Type: Function |
 ``root::float?`` | ``Usage: (float? expression)`` |

<span style="padding-left: 5px">True if the expression is a float, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (float? 1.5))<br>
(test::assert-false (float? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::func?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``func?``](#root::func?-contents) | Type: Macro |
 ``root::func?`` | ``Usage: (func? to-test)`` |

<span style="padding-left: 5px">True if the expression is a [builtin?](#root::builtin?), a [lambda?](#root::lambda?), or a [macro?](#root::macro?)
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def func?-test 1)<br>
(test::assert-false (func? func?-test))<br>
(test::assert-true (func? car))<br>
(test::assert-true (func? first))<br>
(test::assert-true (func? let))<br>
<br>
</code>
</details>
<br>


| <a id="root::hash?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``hash?``](#root::hash?-contents) | Type: Function |
 ``root::hash?`` | ``Usage: (hash? expression)`` |

<span style="padding-left: 5px">True if the expression is a hash map, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (hash? (make-hash)) "make-vec")<br>
(test::assert-false (hash? 1))<br>
(test::assert-false (hash? '(1 2 3)))<br>
(test::assert-false (hash? (list)))<br>
(test::assert-false (hash? (vec)))<br>
<br>
</code>
</details>
<br>


| <a id="root::int->float" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``int->float``](#root::int->float-contents) | Type: Function |
 ``root::int->float`` | ``Usage: (int->float int) -> float`` |

<span style="padding-left: 5px">Cast an int as a float.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 0 (int->float 0))<br>
(test::assert-equal 10 (int->float 10))<br>
(test::assert-equal -101 (int->float -101))<br>
(test::assert-error (int->float "not int"))<br>
<br>
</code>
</details>
<br>


| <a id="root::int?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``int?``](#root::int?-contents) | Type: Function |
 ``root::int?`` | ``Usage: (int? expression)`` |

<span style="padding-left: 5px">True if the expression is an int, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (int? 1))<br>
(test::assert-false (int? 1.5))<br>
<br>
</code>
</details>
<br>


| <a id="root::lambda?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``lambda?``](#root::lambda?-contents) | Type: Function |
 ``root::lambda?`` | ``Usage: (lambda? expression)`` |

<span style="padding-left: 5px">True if the expression is a lambda, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (lambda? (fn () ())))<br>
(test::assert-true (lambda? caar))<br>
(test::assert-false (lambda? 1))<br>
(test::assert-false (lambda? if))<br>
<br>
</code>
</details>
<br>


| <a id="root::list?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``list?``](#root::list?-contents) | Type: Function |
 ``root::list?`` | ``Usage: (list? expression)`` |

<span style="padding-left: 5px">True if the expression is a list, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (list? '(1 2 3)) "reader macro")<br>
(test::assert-true (list? (list 1 2 3)) "list")<br>
(test::assert-false (list? 1))<br>
(test::assert-false (list? '#(1 2 3)))<br>
(test::assert-false (list? (vec)))<br>
(test::assert-false (list? '(1 . 2)))<br>
<br>
</code>
</details>
<br>


| <a id="root::macro?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``macro?``](#root::macro?-contents) | Type: Function |
 ``root::macro?`` | ``Usage: (macro? expression)`` |

<span style="padding-left: 5px">True if the expression is a macro, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (macro? (macro () ())))<br>
(test::assert-true (macro? defn))<br>
(test::assert-false (macro? 1))<br>
(test::assert-false (macro? if))<br>
<br>
</code>
</details>
<br>


| <a id="root::nil?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``nil?``](#root::nil?-contents) | Type: Function |
 ``root::nil?`` | ``Usage: (nil? expression)`` |

<span style="padding-left: 5px">True if the expression is nil, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (nil? nil))<br>
(test::assert-false (nil? #t))<br>
<br>
</code>
</details>
<br>


| <a id="root::none?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``none?``](#root::none?-contents) | Type: Function |
 ``root::none?`` | ``Usage: (none? expression)`` |

<span style="padding-left: 5px">True if the expression is nil (aka none/nothing), false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (none? nil))<br>
(test::assert-true (none? '()))<br>
(test::assert-false (none? #t))<br>
(test::assert-false (none? '(1)))<br>
<br>
</code>
</details>
<br>


| <a id="root::pair?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``pair?``](#root::pair?-contents) | Type: Function |
 ``root::pair?`` | ``Usage: (pair? expression)`` |

<span style="padding-left: 5px">True if the expression is a pair, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (pair? '(1 . 2)) "reader macro")<br>
(test::assert-true (pair? (join 1 2)) "join")<br>
(test::assert-true (pair? '(1 2)))<br>
(test::assert-false (pair? 1))<br>
(test::assert-false (pair? '#(1 2 3)))<br>
(test::assert-false (pair? (vec)))<br>
<br>
</code>
</details>
<br>


| <a id="root::process?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``process?``](#root::process?-contents) | Type: Function |
 ``root::process?`` | ``Usage: (process? expression)`` |

<span style="padding-left: 5px">True if the expression is a process, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (process? (syscall 'true)))<br>
(test::assert-true (process? (fork ((fn () nil)))))<br>
(test::assert-false (process? (fn () ())))<br>
(test::assert-false (process? caar))<br>
(test::assert-false (process? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::some?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``some?``](#root::some?-contents) | Type: Function |
 ``root::some?`` | ``Usage: (some? expression)`` |

<span style="padding-left: 5px">True if the expression is NOT nil (aka something), false otherwise.
Note that anything other then nil (including false) is something.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-false (some? nil))<br>
(test::assert-false (some? '()))<br>
(test::assert-true (some? #t))<br>
(test::assert-true (some? '(1)))<br>
<br>
</code>
</details>
<br>


| <a id="root::str->float" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str->float``](#root::str->float-contents) | Type: Function |
 ``root::str->float`` | ``Usage: (str->float string) -> float`` |

<span style="padding-left: 5px">If string is a valid representation of a float return that float.  Error if not.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 0 (str->float "0"))<br>
(test::assert-equal 10.0 (str->float "10.0"))<br>
(test::assert-equal 10.5 (str->float "10.5"))<br>
(test::assert-equal 101 (str->float "101"))<br>
(test::assert-equal -101.95 (str->float "-101.95"))<br>
(test::assert-error (str->float "not int"))<br>
(test::assert-error (str->float "--10"))<br>
<br>
</code>
</details>
<br>


| <a id="root::str->int" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``str->int``](#root::str->int-contents) | Type: Function |
 ``root::str->int`` | ``Usage: (str->int string) -> int`` |

<span style="padding-left: 5px">If string is a valid representation of an integer return that int.  Error if not.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 0 (str->int "0"))<br>
(test::assert-equal 101 (str->int "101"))<br>
(test::assert-equal -101 (str->int "-101"))<br>
(test::assert-error (str->int "not int"))<br>
(test::assert-error (str->int "10.0"))<br>
(test::assert-error (str->int "--10"))<br>
<br>
</code>
</details>
<br>


| <a id="root::string?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``string?``](#root::string?-contents) | Type: Function |
 ``root::string?`` | ``Usage: (string? expression)`` |

<span style="padding-left: 5px">True if the expression is a string, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (string? "string"))<br>
(test::assert-false (string? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::sym" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sym``](#root::sym-contents) | Type: Function |
 ``root::sym`` | ``Usage: (sym expression+) -> symbol`` |

<span style="padding-left: 5px">Takes one or more forms, converts them to strings, concatonates them and returns
a symbol with that name.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-to-symbol-sym nil)<br>
(test::assert-true (symbol? (sym 55)))<br>
(test::assert-true (symbol? (sym 55.0)))<br>
(test::assert-true (symbol? (sym "to-symbol-test-new-symbol")))<br>
(test::assert-true (symbol? (sym (str "to-symbol-test-new-symbol-buf"))))<br>
(test::assert-true (symbol? (sym 'test-to-symbol-sym)))<br>
(set! test-to-symbol-sym "testing-sym")<br>
(test::assert-equal "testing-sym" (sym->str (sym test-to-symbol-sym)))<br>
(test::assert-true (symbol? (sym (sym->str 'test-to-symbol-sym))))<br>
<br>
</code>
</details>
<br>


| <a id="root::sym->str" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``sym->str``](#root::sym->str-contents) | Type: Function |
 ``root::sym->str`` | ``Usage: (sym->str symbol) -> string`` |

<span style="padding-left: 5px">Convert a symbol to the string representation representation of it's name.
The string will be the symbol name as a string.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-sym->str-sym nil)<br>
(test::assert-true (string? (sym->str 'test-sym->str-sym)))<br>
(test::assert-equal "test-sym->str-sym" (sym->str 'test-sym->str-sym))<br>
<br>
</code>
</details>
<br>


| <a id="root::symbol?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``symbol?``](#root::symbol?-contents) | Type: Function |
 ``root::symbol?`` | ``Usage: (symbol? expression)`` |

<span style="padding-left: 5px">True if the expression is a symbol, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (symbol? 'symbol))<br>
(test::assert-false (symbol? 1))<br>
<br>
</code>
</details>
<br>


| <a id="root::true?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``true?``](#root::true?-contents) | Type: Function |
 ``root::true?`` | ``Usage: (true? expression)`` |

<span style="padding-left: 5px">True if the expression is true(#t) (true type NOT non-nil), false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (true? #t))<br>
(test::assert-false (true? #f))<br>
(test::assert-false (true? nil))<br>
(test::assert-false (true? 1))<br>
(test::assert-false (true? "str"))<br>
<br>
</code>
</details>
<br>


| <a id="root::type" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``type``](#root::type-contents) | Type: Function |
 ``root::type`` | ``Usage: (type expression)`` |

<span style="padding-left: 5px">Return the type of the given expression as a string.
Types are:
True
False
Float
Int
Symbol
String
Char
Lambda
Macro
Process
SpecialForm
Function
Vector
Pair
Nil
HashMap
File
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal "True" (type #t))<br>
(test::assert-equal "False" (type #f))<br>
(test::assert-equal "Float" (type 1.1))<br>
(test::assert-equal "Int" (type 1))<br>
(test::assert-equal "Symbol" (type 'symbol))<br>
(def type-sym 'symbol)<br>
(test::assert-equal "Symbol" (type type-sym))<br>
(test::assert-equal "String" (type "string"))<br>
(test::assert-equal "Char" (type #\a))<br>
(test::assert-equal "Lambda" (type (fn () ())))<br>
(test::assert-equal "Macro" (type (macro () ())))<br>
(test::assert-equal "Process" (type (syscall 'true)))<br>
(test::assert-equal "SpecialForm" (type if))<br>
(test::assert-equal "Function" (type type))<br>
(test::assert-equal "Vector" (type '#(1 2 3)))<br>
(def type-vec '#(4 5 6))<br>
(test::assert-equal "Vector" (type type-vec))<br>
(test::assert-equal "Pair" (type '(1 . 2)))<br>
(test::assert-equal "Pair" (type '(1 2 3)))<br>
(test::assert-equal "Nil" (type nil))<br>
(test::assert-equal "Nil" (type '()))<br>
(test::assert-equal "HashMap" (type (make-hash)))<br>
(test::assert-equal "File" (type (open :stdin)))<br>
<br>
</code>
</details>
<br>


| <a id="root::values?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``values?``](#root::values?-contents) | Type: Function |
 ``root::values?`` | ``Usage: (values? expression)`` |

<span style="padding-left: 5px">True if the expression is multi values object, false otherwise.
NOTE: A values object will ALSO be the type of its first value.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (values? (values 1 "str" 5.5)))<br>
(test::assert-false (values? '(1 2 3)))<br>
(test::assert-false (values? '(1 . 3)))<br>
(test::assert-false (values? 1))<br>
(test::assert-true (int? (values 1 "str" 5.5)))<br>
(test::assert-false (string? (values 1 "str" 5.5)))<br>
(test::assert-false (float? (values 1 "str" 5.5)))<br>
(def test-is-values (values 1 2 3 "string" 1.5))<br>
(test::assert-true (values? test-is-values))<br>
(test::assert-true (int? test-is-values))<br>
(test::assert-false (string? test-is-values))<br>
(test::assert-false (float? test-is-values))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec?``](#root::vec?-contents) | Type: Function |
 ``root::vec?`` | ``Usage: (vec? expression)`` |

<span style="padding-left: 5px">True if the expression is a vector, false otherwise.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (vec? '#(1 2 3)) "reader macro")<br>
(test::assert-true (vec? (make-vec)) "make-vec")<br>
(test::assert-true (vec? (vec 1 2 3)) "vec")<br>
(test::assert-false (vec? 1))<br>
(test::assert-false (vec? '(1 2 3)))<br>
(test::assert-false (vec? (list)))<br>
<br>
</code>
</details>
<br>
### <a id="vector-body" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[Vector forms](#vector-contents)
Forms ending in '!' are destructive and change the underlying vector, other forms
do not make changes to the the provided vector.  They are usable in place of a
list for purposes of lambda calls, parameters, etc (they work the same as a list
made from pairs but are vectors not linked lists).  Use #() to declare them in
code (i.e. '#(1 2 3) or #(+ 1 2)).


| <a id="root::make-vec" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``make-vec``](#root::make-vec-contents) | Type: Function |
 ``root::make-vec`` | ``Usage: (make-vec capacity default)`` |

<span style="padding-left: 5px">Make a new vector with capacity and default item(s).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '() (make-vec))<br>
(test::assert-equal '(x x x) (make-vec 3 'x))<br>
(test::assert-equal '(nil nil nil nil nil) (make-vec 5 nil))<br>
(test::assert-equal '() (make-vec 5))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec``](#root::vec-contents) | Type: Function |
 ``root::vec`` | ``Usage: (vec item1 item2 .. itemN)`` |

<span style="padding-left: 5px">Make a new vector with items.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '() (vec))<br>
(test::assert-equal '(1 2 3) (vec 1 2 3))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-clear!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-clear!``](#root::vec-clear!-contents) | Type: Function |
 ``root::vec-clear!`` | ``Usage: (vec-clear! vector)`` |

<span style="padding-left: 5px">Clears a vector.  This is destructive!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-clear-vec (vec 1 2 3))<br>
(test::assert-false (vec-empty? test-clear-vec))<br>
(vec-clear! test-clear-vec)<br>
(test::assert-true (vec-empty? test-clear-vec))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-empty?" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-empty?``](#root::vec-empty?-contents) | Type: Function |
 ``root::vec-empty?`` | ``Usage: (vec-empty? vector)`` |

<span style="padding-left: 5px">True if the vector is empty.
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-true (vec-empty? '#()))<br>
(test::assert-false (vec-empty? '#(1 2 3)))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-insert!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-insert!``](#root::vec-insert!-contents) | Type: Function |
 ``root::vec-insert!`` | ``Usage: (vec-insert! vector index new-element) -> vector`` |

<span style="padding-left: 5px">Inserts new-element at index and moves following elements right in vector.  This is destructive!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-insert-nth-vec (vec 1 2 3))<br>
(test::assert-equal '(1 2 3) test-insert-nth-vec)<br>
(vec-insert! test-insert-nth-vec 1 5)<br>
(test::assert-equal '(1 5 2 3) test-insert-nth-vec)<br>
(vec-insert! test-insert-nth-vec 2 6)<br>
(test::assert-equal '(1 5 6 2 3) test-insert-nth-vec)<br>
(vec-insert! test-insert-nth-vec 0 4)<br>
(test::assert-equal '(4 1 5 6 2 3) test-insert-nth-vec)<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-nth" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-nth``](#root::vec-nth-contents) | Type: Function |
 ``root::vec-nth`` | ``Usage: (vec-nth vector index) -> object`` |

<span style="padding-left: 5px">Get the nth element (0 based) of a vector. If you need the equivalent operation
on a list use [nth](root::nth).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal 5 (vec-nth '#(1 2 3 4 5 6) 4))<br>
(test::assert-equal 1 (vec-nth '#(1 2 3 4 5 6) 0))<br>
(test::assert-equal 3 (vec-nth '#(1 2 3 4 5 6) 2))<br>
(test::assert-equal 6 (vec-nth '#(1 2 3 4 5 6) 5))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-pop!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-pop!``](#root::vec-pop!-contents) | Type: Function |
 ``root::vec-pop!`` | ``Usage: (vec-pop! vector) -> object`` |

<span style="padding-left: 5px">Pops the last object off of the end of the vector.  This is destructive!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-pop-vec (vec 1 2 3))<br>
(test::assert-equal 3 (vec-pop! test-pop-vec))<br>
(test::assert-equal '(1 2) test-pop-vec)<br>
(test::assert-equal 2 (vec-pop! test-pop-vec))<br>
(test::assert-equal '(1) test-pop-vec)<br>
(test::assert-equal 1 (vec-pop! test-pop-vec))<br>
(test::assert-equal '() test-pop-vec)<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-push!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-push!``](#root::vec-push!-contents) | Type: Function |
 ``root::vec-push!`` | ``Usage: (vec-push! vector object) -> vector`` |

<span style="padding-left: 5px">Pushes the provided object onto the end of the vector.  This is destructive!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-push-vec (vec))<br>
(test::assert-equal '(1) (vec-push! test-push-vec 1))<br>
(test::assert-equal '(1) test-push-vec)<br>
(test::assert-equal '(1 2) (vec-push! test-push-vec 2))<br>
(test::assert-equal '(1 2) test-push-vec)<br>
(test::assert-equal '(1 2 3) (vec-push! test-push-vec 3))<br>
(test::assert-equal '(1 2 3) test-push-vec)<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-remove!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-remove!``](#root::vec-remove!-contents) | Type: Function |
 ``root::vec-remove!`` | ``Usage: (vec-remove! vector index) -> vector`` |

<span style="padding-left: 5px">Remove the element at index from vector, shifting all elements after it to the left.
This is destructive!
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-remove-nth-vec (vec 1 2 3))<br>
(test::assert-equal '(1 2 3) test-remove-nth-vec)<br>
(vec-remove! test-remove-nth-vec 1)<br>
(test::assert-equal '(1 3) test-remove-nth-vec)<br>
(vec-remove! test-remove-nth-vec 1)<br>
(test::assert-equal '(1) test-remove-nth-vec)<br>
(vec-remove! test-remove-nth-vec 0)<br>
(test::assert-equal '() test-remove-nth-vec)<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-set!" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-set!``](#root::vec-set!-contents) | Type: Function |
 ``root::vec-set!`` | ``Usage: (vec-set! vector index value) -> vector`` |

<span style="padding-left: 5px">Set the nth index (0 based) of a vector to value. This is destructive! If you
need the equivalent operation on a list use [setnth!](root::setnth!).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(def test-setnth-vec (vec 1 2 3))<br>
(test::assert-equal '(1 5 3) (vec-set! test-setnth-vec 1 5))<br>
(test::assert-equal '(7 5 3) (vec-set! test-setnth-vec 0 7))<br>
(test::assert-equal '(7 5 9) (vec-set! test-setnth-vec 2 9))<br>
<br>
</code>
</details>
<br>


| <a id="root::vec-slice" class="anchor" aria-hidden="true" href="#sl-sh-form-documentation"></a>[``vec-slice``](#root::vec-slice-contents) | Type: Function |
 ``root::vec-slice`` | ``Usage: (vec-slice vector start end?)`` |

<span style="padding-left: 5px">Returns a slice of a vector (0 based indexes, end is exclusive).
</span>
<details style="cursor: pointer; padding-bottom: 15px; padding-left: 10px">
<code>
Example:<br>
(test::assert-equal '(5 6) (vec-slice '#(1 2 3 4 5 6) 4 6))<br>
(test::assert-equal '(1 2 3) (vec-slice '#(1 2 3 4 5 6) 0 3))<br>
(test::assert-equal '(3 4 5) (vec-slice '#(1 2 3 4 5 6) 2 5))<br>
(test::assert-equal '(3 4 5 6) (vec-slice '#(1 2 3 4 5 6) 2))<br>
<br>
</code>
</details>
<br>

version: sl-sh 0.9.58 (master:068f12b, release build, linux [x86_64], Jul 04 2021, 18:14:45 UTC [rustc 1.53.0 (53cb7b09b 2021-06-17)])

