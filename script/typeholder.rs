pub trait ServoParserTrait {

}

pub trait TypeHolderTrait {
    type ServoParserType: ServoParserTrait;
}
