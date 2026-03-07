use anchor_lang::prelude::*;

// IMPORTANTE: Dale a "Build" en el Playground y pega el ID generado aquí
declare_id!("Pega_Tu_Program_ID_Aqui");

#[program]
pub mod refaccionaria_solana {
    use super::*;

    // CREATE: Registrar una nueva refacción
    pub fn registrar_refaccion(
        ctx: Context<RegistrarRefaccion>, 
        nombre: String, 
        marca: String, 
        cantidad: u64
    ) -> Result<()> {
        let refaccion = &mut ctx.accounts.refaccion;
        refaccion.owner = *ctx.accounts.owner.key;
        refaccion.nombre = nombre;
        refaccion.marca = marca;
        refaccion.cantidad = cantidad;
        refaccion.disponible = true;
        
        msg!("Refacción {} registrada con éxito!", refaccion.nombre);
        Ok(())
    }

    // UPDATE: Actualizar el stock o disponibilidad
    pub fn actualizar_stock(ctx: Context<ActualizarRefaccion>, nueva_cantidad: u64) -> Result<()> {
        let refaccion = &mut ctx.accounts.refaccion;
        refaccion.cantidad = nueva_cantidad;
        refaccion.disponible = nueva_cantidad > 0;
        msg!("Stock actualizado. Cantidad actual: {}", refaccion.cantidad);
        Ok(())
    }

    // DELETE: Eliminar registro y recuperar el SOL del espacio (Rent)
    pub fn eliminar_refaccion(_ctx: Context<EliminarRefaccion>) -> Result<()> {
        msg!("Registro de refacción eliminado.");
        Ok(())
    }
}

// Estructura de los datos almacenados
#[account]
#[derive(InitSpace)] // Requisito para calcular espacio eficientemente
pub struct Refaccion {
    pub owner: Pubkey,
    #[max_len(32)]
    pub nombre: String,
    #[max_len(32)]
    pub marca: String,
    pub cantidad: u64,
    pub disponible: bool,
}

// Contextos con lógica de PDA
#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct RegistrarRefaccion<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + Refaccion::INIT_SPACE,
        // Semilla: "refaccion" + llave del dueño + nombre del producto
        seeds = [b"refaccion", owner.key().as_ref(), nombre.as_bytes()],
        bump
    )]
    pub refaccion: Account<'info, Refaccion>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarRefaccion<'info> {
    #[account(mut, has_one = owner)] // Seguridad: Solo el dueño puede editar
    pub refaccion: Account<'info, Refaccion>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarRefaccion<'info> {
    #[account(mut, has_one = owner, close = owner)] // Cierra la cuenta y devuelve el SOL
    pub refaccion: Account<'info, Refaccion>,
    pub owner: Signer<'info>,
}
