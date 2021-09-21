#[derive(Debug, PartialEq)]
pub struct Dna {
    inner: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    inner: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Ok(Dna {
            inner: dna
                .to_lowercase()
                .char_indices()
                .map(|(idx, c)| match c {
                    'a' | 'c' | 'g' | 't' => Ok(c),
                    _ => Err(idx),
                })
                .collect::<Result<String, _>>()?,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(
            self.inner
                .chars()
                .map(|c| match c {
                    'a' => 'u',
                    'c' => 'g',
                    'g' => 'c',
                    't' => 'a',
                    _ => unreachable!(),
                })
                .collect::<String>()
                .as_str(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        Ok(Rna {
            inner: rna
                .to_lowercase()
                .char_indices()
                .map(|(idx, c)| match c {
                    'a' | 'c' | 'g' | 'u' => Ok(c),
                    _ => Err(idx),
                })
                .collect::<Result<String, _>>()?,
        })
    }
}
