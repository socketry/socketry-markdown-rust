// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use super::{
    check_bullet::check_bullet, check_bullet_ordered::check_bullet_ordered,
    check_bullet_other::check_bullet_other, check_emphasis::check_emphasis,
    check_fence::check_fence, check_quote::check_quote, check_rule::check_rule,
    check_rule_repetition::check_rule_repetition, check_strong::check_strong,
};
use crate::markdown::{state::State, Options};

#[test]
fn validates_markdown_serialization_markers() {
    let options = Options::default();
    let mut state = State::new(&options);
    assert_eq!(check_bullet(&mut state).unwrap(), '-');
    assert_eq!(check_bullet_ordered(&mut state).unwrap(), '.');
    assert_eq!(check_bullet_other(&mut state).unwrap(), '*');
    assert_eq!(check_emphasis(&state).unwrap(), '*');
    assert_eq!(check_fence(&mut state).unwrap(), '`');
    assert_eq!(check_quote(&state).unwrap(), '"');
    assert_eq!(check_rule(&state).unwrap(), '*');
    assert_eq!(check_rule_repetition(&state).unwrap(), 3);
    assert_eq!(check_strong(&state).unwrap(), '*');

    for bullet in ['+', '-'] {
        let options = Options {
            bullet,
            ..Options::default()
        };
        assert_eq!(check_bullet(&mut State::new(&options)).unwrap(), bullet);
    }

    for bullet in ['.', ')'] {
        let options = Options {
            bullet_ordered: bullet,
            ..Options::default()
        };
        assert_eq!(
            check_bullet_ordered(&mut State::new(&options)).unwrap(),
            bullet
        );
    }

    let options = Options {
        bullet: '+',
        bullet_other: '-',
        ..Options::default()
    };
    assert_eq!(check_bullet_other(&mut State::new(&options)).unwrap(), '*');

    for marker in ['_', '*'] {
        let options = Options {
            emphasis: marker,
            strong: marker,
            ..Options::default()
        };
        let state = State::new(&options);
        assert_eq!(check_emphasis(&state).unwrap(), marker);
        assert_eq!(check_strong(&state).unwrap(), marker);
    }

    let options = Options {
        fence: '~',
        quote: '\'',
        rule: '-',
        rule_repetition: 4,
        ..Options::default()
    };
    let state = State::new(&options);
    assert_eq!(check_fence(&mut State::new(&options)).unwrap(), '~');
    assert_eq!(check_quote(&state).unwrap(), '\'');
    assert_eq!(check_rule(&state).unwrap(), '-');
    assert_eq!(check_rule_repetition(&state).unwrap(), 4);

    let invalid = Options {
        bullet: 'x',
        ..Options::default()
    };
    assert!(check_bullet(&mut State::new(&invalid)).is_err());

    let invalid = Options {
        bullet_ordered: 'x',
        ..Options::default()
    };
    assert!(check_bullet_ordered(&mut State::new(&invalid)).is_err());

    let invalid = Options {
        bullet: '*',
        bullet_other: 'x',
        ..Options::default()
    };
    assert!(check_bullet_other(&mut State::new(&invalid)).is_err());

    let invalid = Options {
        bullet: 'x',
        bullet_other: '-',
        ..Options::default()
    };
    assert!(check_bullet_other(&mut State::new(&invalid)).is_err());

    let invalid = Options {
        bullet: '*',
        bullet_other: '*',
        ..Options::default()
    };
    assert!(check_bullet_other(&mut State::new(&invalid)).is_err());

    for options in [
        Options {
            emphasis: 'x',
            ..Options::default()
        },
        Options {
            strong: 'x',
            ..Options::default()
        },
        Options {
            fence: 'x',
            ..Options::default()
        },
        Options {
            quote: 'x',
            ..Options::default()
        },
        Options {
            rule: 'x',
            ..Options::default()
        },
        Options {
            rule_repetition: 2,
            ..Options::default()
        },
    ] {
        let mut state = State::new(&options);
        assert!(check_emphasis(&state).is_err() || options.emphasis == '*');
        assert!(check_strong(&state).is_err() || options.strong == '*');
        assert!(check_fence(&mut state).is_err() || options.fence == '`');
        assert!(check_quote(&state).is_err() || options.quote == '"');
        assert!(check_rule(&state).is_err() || options.rule == '*');
        assert!(check_rule_repetition(&state).is_err() || options.rule_repetition == 3);
    }
}
