use bullistic_candy_machine::cmp_pubkeys;
pub use mpl_token_metadata::state::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH};

use crate::{config::Creator, validate::errors::ValidateParserError};

pub fn check_name(name: &str) -> Result<(), ValidateParserError> {
    if name.len() > MAX_NAME_LENGTH {
        return Err(ValidateParserError::NameTooLong);
    }
    Ok(())
}

pub fn check_symbol(symbol: &str) -> Result<(), ValidateParserError> {
    if symbol.len() > MAX_SYMBOL_LENGTH {
        return Err(ValidateParserError::SymbolTooLong);
    }
    Ok(())
}

pub fn check_url(url: &str) -> Result<(), ValidateParserError> {
    if url.len() > MAX_URI_LENGTH {
        return Err(ValidateParserError::UrlTooLong);
    }
    Ok(())
}

pub fn check_seller_fee_basis_points(
    seller_fee_basis_points: u16,
) -> Result<(), ValidateParserError> {
    if seller_fee_basis_points > 10000 {
        return Err(ValidateParserError::InvalidSellerFeeBasisPoints(
            seller_fee_basis_points,
        ));
    }
    Ok(())
}

pub fn check_creators_shares(creators: &[Creator]) -> Result<(), ValidateParserError> {
    let mut shares = 0;
    for creator in creators {
        shares += creator.share;
    }

    if shares != 100 {
        return Err(ValidateParserError::InvalidCreatorShare);
    }
    Ok(())
}

pub fn validate_metadata_creators(
    config_creators: &Vec<Creator>,
    metadata_creators: &Vec<Creator>,
) -> Result<(), ValidateParserError> {
    if config_creators.len() != metadata_creators.len() {
        return Err(ValidateParserError::ConfigCreatorMismatch);
    }

    for (i, config_creator) in config_creators.iter().enumerate() {
        let metadata_creator = &metadata_creators[i];
        if !cmp_pubkeys(&config_creator.address, &metadata_creator.address) {
            return Err(ValidateParserError::ConfigCreatorMismatch);
        }

        if config_creator.share != metadata_creator.share {
            return Err(ValidateParserError::ConfigCreatorMismatch);
        }
    }

    Ok(())
}
