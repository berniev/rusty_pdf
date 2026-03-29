/*
 * Document Catalog:
 *     The primary dictionary object containing references directly or indirectly to all other
 *     objects in the document with the exception that there may be objects in the trailer that
 *     are not referred to by the Catalog
 *
 *  Catalog
 *          Page Tree
 *                           Page
 *                                          Content Stream
 *                                          Thumbnail Image
 *                                          Annotations
 *                                    ...
 *                           Page
 *          Outline Hierachy
 *                           Outline Entry
 *                                ...
 *                           Outline Entry
 *          Article Threads
 *                           Thread
 *                                          Bead <--> Bead
 *                               ...
 *                           Thread
 *          Named Destinations
 *          Interactive form
 *
 * Entries:
 *     Type               name           Reqd          "Catalog"
 *     Version            name           Opt     1.4
 *     Extensions         dictionary     Opt
 *     Pages              dictionary     Reqd          shall be indirect ref
 *     PageLabels         number tree    Opt     1.3
 *     Names              dictionary     Opt     1.2
 *     Dests              dictionary     Opt     1.1   indirect reference
 *     ViewerPreferences  dictionary     Opt     1.2
 *     PageLayout         name           Opt
 *         SinglePage (def)
 *         OneColumn
 *         TwoColumnLeft
 *         TwoColumnRight
 *         TwoPageLeft
 *         TwoPageRight
 *     PageMode           name           Opt
 *          UseNone (def)
 *          UseOutlines
 *          UseThumbs
 *          FullScreen
 *          UseOC
 *          UseAttachments
 *     Outlines            dictionary     Opt         indirect reference
 *     Threads             array          Opt    1.1  indirect reference
 *     OpenAction          array or dict  Opt    1.1
 *     AA                  dictionary     Opt    1.4
 *     URI                 dictionary     Opt    1.1
 *     AcroForm            dictionary     Opt    1.2
 *     Metadata            dictionary     Opt    1.4
 *     StructTreeRoot      dictionary     Opt    1.3
 *     MarkInfo            dictionary     Opt    1.4
 *     Lang                text string    Opt    1.4
 *     SpiderInfo          dictionary     Opt    1.3
 *     OutputIntents       array          Opt    1.4
 *     PieceInfo           dictionary     Opt    1.4
 *     OCProperties        dictionary     Opt    1.5
 *     Perms               dictionary     Opt    1.5
 *     Legal               dictionary     Opt    1.5
 *     Requirements        array          Opt    1.7
 *     Collection          dictionary     Opt    1.7
 *     NeedsRendering      boolean        Opt    1.7
 */

/*
The Catalog is itself an indirect object in the body — the trailer points to it by object number.
It's a dictionary containing a mix of:
- **Direct values** — simple like `/Type /Catalog`, `/PageLayout /SinglePage`, `/Lang (en-GB)`
- **Indirect references** — pointers to other indirect objects in the body, written as `N 0 R`

So a minimal real catalog in the file looks like:
1 0 obj
<<
  /Type /Catalog
  /Pages 2 0 R        ← indirect ref — Pages tree is its own body object
  /PageLayout /SinglePage  ← direct name, lives right here
>>
endobj

certain entries must be indirect references:
/Pages — the page tree root must be indirect
/Outlines — must be indirect
/Metadata — must be indirect (it's a stream, streams are always indirect)
Others may be either — small dictionaries like /ViewerPreferences can be direct (embedded inline)
or indirect (separate body object). The spec leaves it to the writer.
So the catalog dictionary itself is direct content inside its own indirect object wrapper — and
everything it points to with N 0 R are separate indirect objects in the body, each with their own
XRef entry.

The catalog dictionary contains only:
    Direct objects — names, booleans, strings, and small dictionaries/arrays embedded inline
    Indirect references (N 0 R) — pointers to indirect objects defined elsewhere in the body
The actual obj...endobj definitions are never inside the catalog — they're always elsewhere in the
body. The catalog just holds the references to find them.

So the catalog is essentially a directory — it tells you where to find things, it doesn't contain
the things themselves (beyond trivial values).
*/

/*        
"AA",                1.4
"AcroForm",          1.2
"Collection",        1.7
"Dests",             1.1
"Extensions",        1.3
"Lang",              1.4
"Legal",             1.5
"MarkInfo",          1.4
"Metadata",          1.4
"Names",             1.2
"NeedsRendering",    1.7
"OCProperties",      1.5
"OpenAction",        1.1
"Outlines",          1.1
"OutputIntents",     1.4
"PageLabels",        1.3
"PageLayout",        1.0
"Pages",             1.0
"PageMode",          1.0
"Perms",             1.5
"PieceInfo",         1.4
"Requirements",      1.7
"SpiderInfo",        1.3
"StructTreeRoot",    1.3
"Threads",           1.1
"Type",              1.0
"URI",               1.1
"Version",           1.4
"ViewerPreferences", 1.2
*/
