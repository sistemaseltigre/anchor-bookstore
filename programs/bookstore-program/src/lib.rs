use anchor_lang::prelude::*;

declare_id!("DT7vPBkn8xKNTpQVFHUqQa4dJkGjS3CWThwvF17KUUdE");

#[program]
pub mod bookstore_program {
    use super::*;

    pub fn add_book(
        ctx: Context<AddBook>,
        id: u64,
        title: String,
        author: String,
        price: f32,
        categories: String,
        cant: u64
    ) -> Result<()> {
        msg!("Book added");
        msg!("Title: {}", title);
        msg!("Author: {}", author);
        msg!("Price: {}", price);
        msg!("Categories: {}", categories);
        msg!("Cant: {}", cant);

        let book = &mut ctx.accounts.book;
        book.id = id;
        book.title = title;
        book.author = author;
        book.price = price;
        book.categories = categories;
        book.cant = cant;

        Ok(())
    }

    pub fn update_book(
        ctx: Context<UpdateBook>,
        id: u64,
        title: String,
        author: String,
        price: f32,
        categories: String,
        cant: u64
    ) -> Result<()> {
        msg!("Book updated");
        msg!("Title: {}", title);
        msg!("Author: {}", author);
        msg!("Price: {}", price);
        msg!("Categories: {}", categories);
        msg!("Cant: {}", cant);

        let book = &mut ctx.accounts.book;
        book.title = title;
        book.author = author;
        book.price = price;
        book.categories = categories;
        book.cant = cant;

        Ok(())
    }

    pub fn delete_book(_ctx: Context<DeleteBook>, id: u64) -> Result<()> {
        msg!("Book deleted {}", id);
        Ok(())
    }
}

#[account]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub price: f32,
    pub categories: String,
    pub cant: u64,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct AddBook<'info> {
    #[account(
        init,
        seeds = [id.to_le_bytes().as_ref(), payer.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + 8 + 32 + 32 + 4 + 32 + 8,
    )]
    pub book: Account<'info, Book>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct UpdateBook<'info> {
    #[account(
        mut,
        seeds = [id.to_le_bytes().as_ref(), payer.key().as_ref()],
        bump,
        realloc = 8 + 8 + 32 + 32 + 4 + 32 + 8,
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub book: Account<'info, Book>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct DeleteBook<'info> {
    #[account(
        mut,
        seeds = [&id.to_le_bytes(), payer.key().as_ref()],
        bump,
        close = payer,
    )]
    pub book: Account<'info, Book>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}