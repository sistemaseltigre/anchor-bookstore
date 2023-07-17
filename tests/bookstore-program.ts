import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { BookstoreProgram } from "../target/types/bookstore_program";

describe("BookStore Program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.BookstoreProgram as Program<BookstoreProgram>;

  //Book data
  const book = {
    id: new anchor.BN(40),
    title: "Elchuo160 Hisotry",
    author: "Jesus Silva",
    price: 1000.0,
    categories: "History",
    cant: new anchor.BN(10000),
  }
  
  const [bookPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      book.id.toArrayLike(Buffer, "le", 8), 
      provider.wallet.publicKey.toBuffer()
    ],
    program.programId
  )  
 
  it("Book is added", async () => {
    const tx = await program.methods.addBook(book.id,book.title,book.author,book.price,book.categories,book.cant).accounts({
      book: bookPda,
    }).rpc()
  })

  it("Book is updated", async () => {
    const newPrice = 1500.0;
    const newCant = new anchor.BN(5000);  
    const tx = await program.methods
      .updateBook(book.id,book.title,book.author,newPrice,book.categories,newCant)
      .accounts({
        book: bookPda
      })
      .rpc()

   })

  it("Delete Book", async () => {
    const tx = await program.methods
      .deleteBook(book.id)
      .accounts({
        book: bookPda
      })
      .rpc()
  })
})

