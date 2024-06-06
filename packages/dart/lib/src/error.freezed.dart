// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'error.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$LiquidSdkError {}

/// @nodoc
abstract class $LiquidSdkErrorCopyWith<$Res> {
  factory $LiquidSdkErrorCopyWith(LiquidSdkError value, $Res Function(LiquidSdkError) then) =
      _$LiquidSdkErrorCopyWithImpl<$Res, LiquidSdkError>;
}

/// @nodoc
class _$LiquidSdkErrorCopyWithImpl<$Res, $Val extends LiquidSdkError>
    implements $LiquidSdkErrorCopyWith<$Res> {
  _$LiquidSdkErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LiquidSdkError_AlreadyStartedImplCopyWith<$Res> {
  factory _$$LiquidSdkError_AlreadyStartedImplCopyWith(_$LiquidSdkError_AlreadyStartedImpl value,
          $Res Function(_$LiquidSdkError_AlreadyStartedImpl) then) =
      __$$LiquidSdkError_AlreadyStartedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LiquidSdkError_AlreadyStartedImplCopyWithImpl<$Res>
    extends _$LiquidSdkErrorCopyWithImpl<$Res, _$LiquidSdkError_AlreadyStartedImpl>
    implements _$$LiquidSdkError_AlreadyStartedImplCopyWith<$Res> {
  __$$LiquidSdkError_AlreadyStartedImplCopyWithImpl(
      _$LiquidSdkError_AlreadyStartedImpl _value, $Res Function(_$LiquidSdkError_AlreadyStartedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LiquidSdkError_AlreadyStartedImpl extends LiquidSdkError_AlreadyStarted {
  const _$LiquidSdkError_AlreadyStartedImpl() : super._();

  @override
  String toString() {
    return 'LiquidSdkError.alreadyStarted()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LiquidSdkError_AlreadyStartedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class LiquidSdkError_AlreadyStarted extends LiquidSdkError {
  const factory LiquidSdkError_AlreadyStarted() = _$LiquidSdkError_AlreadyStartedImpl;
  const LiquidSdkError_AlreadyStarted._() : super._();
}

/// @nodoc
abstract class _$$LiquidSdkError_GenericImplCopyWith<$Res> {
  factory _$$LiquidSdkError_GenericImplCopyWith(
          _$LiquidSdkError_GenericImpl value, $Res Function(_$LiquidSdkError_GenericImpl) then) =
      __$$LiquidSdkError_GenericImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LiquidSdkError_GenericImplCopyWithImpl<$Res>
    extends _$LiquidSdkErrorCopyWithImpl<$Res, _$LiquidSdkError_GenericImpl>
    implements _$$LiquidSdkError_GenericImplCopyWith<$Res> {
  __$$LiquidSdkError_GenericImplCopyWithImpl(
      _$LiquidSdkError_GenericImpl _value, $Res Function(_$LiquidSdkError_GenericImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LiquidSdkError_GenericImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LiquidSdkError_GenericImpl extends LiquidSdkError_Generic {
  const _$LiquidSdkError_GenericImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LiquidSdkError.generic(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkError_GenericImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkError_GenericImplCopyWith<_$LiquidSdkError_GenericImpl> get copyWith =>
      __$$LiquidSdkError_GenericImplCopyWithImpl<_$LiquidSdkError_GenericImpl>(this, _$identity);
}

abstract class LiquidSdkError_Generic extends LiquidSdkError {
  const factory LiquidSdkError_Generic({required final String err}) = _$LiquidSdkError_GenericImpl;
  const LiquidSdkError_Generic._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LiquidSdkError_GenericImplCopyWith<_$LiquidSdkError_GenericImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkError_ServiceConnectivityImplCopyWith<$Res> {
  factory _$$LiquidSdkError_ServiceConnectivityImplCopyWith(_$LiquidSdkError_ServiceConnectivityImpl value,
          $Res Function(_$LiquidSdkError_ServiceConnectivityImpl) then) =
      __$$LiquidSdkError_ServiceConnectivityImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$LiquidSdkError_ServiceConnectivityImplCopyWithImpl<$Res>
    extends _$LiquidSdkErrorCopyWithImpl<$Res, _$LiquidSdkError_ServiceConnectivityImpl>
    implements _$$LiquidSdkError_ServiceConnectivityImplCopyWith<$Res> {
  __$$LiquidSdkError_ServiceConnectivityImplCopyWithImpl(_$LiquidSdkError_ServiceConnectivityImpl _value,
      $Res Function(_$LiquidSdkError_ServiceConnectivityImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$LiquidSdkError_ServiceConnectivityImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LiquidSdkError_ServiceConnectivityImpl extends LiquidSdkError_ServiceConnectivity {
  const _$LiquidSdkError_ServiceConnectivityImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'LiquidSdkError.serviceConnectivity(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiquidSdkError_ServiceConnectivityImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiquidSdkError_ServiceConnectivityImplCopyWith<_$LiquidSdkError_ServiceConnectivityImpl> get copyWith =>
      __$$LiquidSdkError_ServiceConnectivityImplCopyWithImpl<_$LiquidSdkError_ServiceConnectivityImpl>(
          this, _$identity);
}

abstract class LiquidSdkError_ServiceConnectivity extends LiquidSdkError {
  const factory LiquidSdkError_ServiceConnectivity({required final String err}) =
      _$LiquidSdkError_ServiceConnectivityImpl;
  const LiquidSdkError_ServiceConnectivity._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$LiquidSdkError_ServiceConnectivityImplCopyWith<_$LiquidSdkError_ServiceConnectivityImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiquidSdkError_NotStartedImplCopyWith<$Res> {
  factory _$$LiquidSdkError_NotStartedImplCopyWith(
          _$LiquidSdkError_NotStartedImpl value, $Res Function(_$LiquidSdkError_NotStartedImpl) then) =
      __$$LiquidSdkError_NotStartedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LiquidSdkError_NotStartedImplCopyWithImpl<$Res>
    extends _$LiquidSdkErrorCopyWithImpl<$Res, _$LiquidSdkError_NotStartedImpl>
    implements _$$LiquidSdkError_NotStartedImplCopyWith<$Res> {
  __$$LiquidSdkError_NotStartedImplCopyWithImpl(
      _$LiquidSdkError_NotStartedImpl _value, $Res Function(_$LiquidSdkError_NotStartedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LiquidSdkError_NotStartedImpl extends LiquidSdkError_NotStarted {
  const _$LiquidSdkError_NotStartedImpl() : super._();

  @override
  String toString() {
    return 'LiquidSdkError.notStarted()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LiquidSdkError_NotStartedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class LiquidSdkError_NotStarted extends LiquidSdkError {
  const factory LiquidSdkError_NotStarted() = _$LiquidSdkError_NotStartedImpl;
  const LiquidSdkError_NotStarted._() : super._();
}

/// @nodoc
mixin _$PaymentError {}

/// @nodoc
abstract class $PaymentErrorCopyWith<$Res> {
  factory $PaymentErrorCopyWith(PaymentError value, $Res Function(PaymentError) then) =
      _$PaymentErrorCopyWithImpl<$Res, PaymentError>;
}

/// @nodoc
class _$PaymentErrorCopyWithImpl<$Res, $Val extends PaymentError> implements $PaymentErrorCopyWith<$Res> {
  _$PaymentErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$PaymentError_AlreadyClaimedImplCopyWith<$Res> {
  factory _$$PaymentError_AlreadyClaimedImplCopyWith(
          _$PaymentError_AlreadyClaimedImpl value, $Res Function(_$PaymentError_AlreadyClaimedImpl) then) =
      __$$PaymentError_AlreadyClaimedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_AlreadyClaimedImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_AlreadyClaimedImpl>
    implements _$$PaymentError_AlreadyClaimedImplCopyWith<$Res> {
  __$$PaymentError_AlreadyClaimedImplCopyWithImpl(
      _$PaymentError_AlreadyClaimedImpl _value, $Res Function(_$PaymentError_AlreadyClaimedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_AlreadyClaimedImpl extends PaymentError_AlreadyClaimed {
  const _$PaymentError_AlreadyClaimedImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.alreadyClaimed()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_AlreadyClaimedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_AlreadyClaimed extends PaymentError {
  const factory PaymentError_AlreadyClaimed() = _$PaymentError_AlreadyClaimedImpl;
  const PaymentError_AlreadyClaimed._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_AmountOutOfRangeImplCopyWith<$Res> {
  factory _$$PaymentError_AmountOutOfRangeImplCopyWith(_$PaymentError_AmountOutOfRangeImpl value,
          $Res Function(_$PaymentError_AmountOutOfRangeImpl) then) =
      __$$PaymentError_AmountOutOfRangeImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_AmountOutOfRangeImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_AmountOutOfRangeImpl>
    implements _$$PaymentError_AmountOutOfRangeImplCopyWith<$Res> {
  __$$PaymentError_AmountOutOfRangeImplCopyWithImpl(
      _$PaymentError_AmountOutOfRangeImpl _value, $Res Function(_$PaymentError_AmountOutOfRangeImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_AmountOutOfRangeImpl extends PaymentError_AmountOutOfRange {
  const _$PaymentError_AmountOutOfRangeImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.amountOutOfRange()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_AmountOutOfRangeImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_AmountOutOfRange extends PaymentError {
  const factory PaymentError_AmountOutOfRange() = _$PaymentError_AmountOutOfRangeImpl;
  const PaymentError_AmountOutOfRange._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_GenericImplCopyWith<$Res> {
  factory _$$PaymentError_GenericImplCopyWith(
          _$PaymentError_GenericImpl value, $Res Function(_$PaymentError_GenericImpl) then) =
      __$$PaymentError_GenericImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$PaymentError_GenericImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_GenericImpl>
    implements _$$PaymentError_GenericImplCopyWith<$Res> {
  __$$PaymentError_GenericImplCopyWithImpl(
      _$PaymentError_GenericImpl _value, $Res Function(_$PaymentError_GenericImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$PaymentError_GenericImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentError_GenericImpl extends PaymentError_Generic {
  const _$PaymentError_GenericImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'PaymentError.generic(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentError_GenericImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentError_GenericImplCopyWith<_$PaymentError_GenericImpl> get copyWith =>
      __$$PaymentError_GenericImplCopyWithImpl<_$PaymentError_GenericImpl>(this, _$identity);
}

abstract class PaymentError_Generic extends PaymentError {
  const factory PaymentError_Generic({required final String err}) = _$PaymentError_GenericImpl;
  const PaymentError_Generic._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$PaymentError_GenericImplCopyWith<_$PaymentError_GenericImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentError_InvalidOrExpiredFeesImplCopyWith<$Res> {
  factory _$$PaymentError_InvalidOrExpiredFeesImplCopyWith(_$PaymentError_InvalidOrExpiredFeesImpl value,
          $Res Function(_$PaymentError_InvalidOrExpiredFeesImpl) then) =
      __$$PaymentError_InvalidOrExpiredFeesImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_InvalidOrExpiredFeesImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_InvalidOrExpiredFeesImpl>
    implements _$$PaymentError_InvalidOrExpiredFeesImplCopyWith<$Res> {
  __$$PaymentError_InvalidOrExpiredFeesImplCopyWithImpl(_$PaymentError_InvalidOrExpiredFeesImpl _value,
      $Res Function(_$PaymentError_InvalidOrExpiredFeesImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_InvalidOrExpiredFeesImpl extends PaymentError_InvalidOrExpiredFees {
  const _$PaymentError_InvalidOrExpiredFeesImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.invalidOrExpiredFees()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_InvalidOrExpiredFeesImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_InvalidOrExpiredFees extends PaymentError {
  const factory PaymentError_InvalidOrExpiredFees() = _$PaymentError_InvalidOrExpiredFeesImpl;
  const PaymentError_InvalidOrExpiredFees._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_InsufficientFundsImplCopyWith<$Res> {
  factory _$$PaymentError_InsufficientFundsImplCopyWith(_$PaymentError_InsufficientFundsImpl value,
          $Res Function(_$PaymentError_InsufficientFundsImpl) then) =
      __$$PaymentError_InsufficientFundsImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_InsufficientFundsImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_InsufficientFundsImpl>
    implements _$$PaymentError_InsufficientFundsImplCopyWith<$Res> {
  __$$PaymentError_InsufficientFundsImplCopyWithImpl(
      _$PaymentError_InsufficientFundsImpl _value, $Res Function(_$PaymentError_InsufficientFundsImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_InsufficientFundsImpl extends PaymentError_InsufficientFunds {
  const _$PaymentError_InsufficientFundsImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.insufficientFunds()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_InsufficientFundsImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_InsufficientFunds extends PaymentError {
  const factory PaymentError_InsufficientFunds() = _$PaymentError_InsufficientFundsImpl;
  const PaymentError_InsufficientFunds._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_InvalidInvoiceImplCopyWith<$Res> {
  factory _$$PaymentError_InvalidInvoiceImplCopyWith(
          _$PaymentError_InvalidInvoiceImpl value, $Res Function(_$PaymentError_InvalidInvoiceImpl) then) =
      __$$PaymentError_InvalidInvoiceImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_InvalidInvoiceImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_InvalidInvoiceImpl>
    implements _$$PaymentError_InvalidInvoiceImplCopyWith<$Res> {
  __$$PaymentError_InvalidInvoiceImplCopyWithImpl(
      _$PaymentError_InvalidInvoiceImpl _value, $Res Function(_$PaymentError_InvalidInvoiceImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_InvalidInvoiceImpl extends PaymentError_InvalidInvoice {
  const _$PaymentError_InvalidInvoiceImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.invalidInvoice()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_InvalidInvoiceImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_InvalidInvoice extends PaymentError {
  const factory PaymentError_InvalidInvoice() = _$PaymentError_InvalidInvoiceImpl;
  const PaymentError_InvalidInvoice._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_InvalidPreimageImplCopyWith<$Res> {
  factory _$$PaymentError_InvalidPreimageImplCopyWith(
          _$PaymentError_InvalidPreimageImpl value, $Res Function(_$PaymentError_InvalidPreimageImpl) then) =
      __$$PaymentError_InvalidPreimageImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_InvalidPreimageImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_InvalidPreimageImpl>
    implements _$$PaymentError_InvalidPreimageImplCopyWith<$Res> {
  __$$PaymentError_InvalidPreimageImplCopyWithImpl(
      _$PaymentError_InvalidPreimageImpl _value, $Res Function(_$PaymentError_InvalidPreimageImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_InvalidPreimageImpl extends PaymentError_InvalidPreimage {
  const _$PaymentError_InvalidPreimageImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.invalidPreimage()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_InvalidPreimageImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_InvalidPreimage extends PaymentError {
  const factory PaymentError_InvalidPreimage() = _$PaymentError_InvalidPreimageImpl;
  const PaymentError_InvalidPreimage._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_LwkErrorImplCopyWith<$Res> {
  factory _$$PaymentError_LwkErrorImplCopyWith(
          _$PaymentError_LwkErrorImpl value, $Res Function(_$PaymentError_LwkErrorImpl) then) =
      __$$PaymentError_LwkErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$PaymentError_LwkErrorImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_LwkErrorImpl>
    implements _$$PaymentError_LwkErrorImplCopyWith<$Res> {
  __$$PaymentError_LwkErrorImplCopyWithImpl(
      _$PaymentError_LwkErrorImpl _value, $Res Function(_$PaymentError_LwkErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$PaymentError_LwkErrorImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentError_LwkErrorImpl extends PaymentError_LwkError {
  const _$PaymentError_LwkErrorImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'PaymentError.lwkError(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentError_LwkErrorImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentError_LwkErrorImplCopyWith<_$PaymentError_LwkErrorImpl> get copyWith =>
      __$$PaymentError_LwkErrorImplCopyWithImpl<_$PaymentError_LwkErrorImpl>(this, _$identity);
}

abstract class PaymentError_LwkError extends PaymentError {
  const factory PaymentError_LwkError({required final String err}) = _$PaymentError_LwkErrorImpl;
  const PaymentError_LwkError._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$PaymentError_LwkErrorImplCopyWith<_$PaymentError_LwkErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentError_PairsNotFoundImplCopyWith<$Res> {
  factory _$$PaymentError_PairsNotFoundImplCopyWith(
          _$PaymentError_PairsNotFoundImpl value, $Res Function(_$PaymentError_PairsNotFoundImpl) then) =
      __$$PaymentError_PairsNotFoundImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_PairsNotFoundImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_PairsNotFoundImpl>
    implements _$$PaymentError_PairsNotFoundImplCopyWith<$Res> {
  __$$PaymentError_PairsNotFoundImplCopyWithImpl(
      _$PaymentError_PairsNotFoundImpl _value, $Res Function(_$PaymentError_PairsNotFoundImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_PairsNotFoundImpl extends PaymentError_PairsNotFound {
  const _$PaymentError_PairsNotFoundImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.pairsNotFound()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_PairsNotFoundImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_PairsNotFound extends PaymentError {
  const factory PaymentError_PairsNotFound() = _$PaymentError_PairsNotFoundImpl;
  const PaymentError_PairsNotFound._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_PaymentTimeoutImplCopyWith<$Res> {
  factory _$$PaymentError_PaymentTimeoutImplCopyWith(
          _$PaymentError_PaymentTimeoutImpl value, $Res Function(_$PaymentError_PaymentTimeoutImpl) then) =
      __$$PaymentError_PaymentTimeoutImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_PaymentTimeoutImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_PaymentTimeoutImpl>
    implements _$$PaymentError_PaymentTimeoutImplCopyWith<$Res> {
  __$$PaymentError_PaymentTimeoutImplCopyWithImpl(
      _$PaymentError_PaymentTimeoutImpl _value, $Res Function(_$PaymentError_PaymentTimeoutImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_PaymentTimeoutImpl extends PaymentError_PaymentTimeout {
  const _$PaymentError_PaymentTimeoutImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.paymentTimeout()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_PaymentTimeoutImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_PaymentTimeout extends PaymentError {
  const factory PaymentError_PaymentTimeout() = _$PaymentError_PaymentTimeoutImpl;
  const PaymentError_PaymentTimeout._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_PersistErrorImplCopyWith<$Res> {
  factory _$$PaymentError_PersistErrorImplCopyWith(
          _$PaymentError_PersistErrorImpl value, $Res Function(_$PaymentError_PersistErrorImpl) then) =
      __$$PaymentError_PersistErrorImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PaymentError_PersistErrorImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_PersistErrorImpl>
    implements _$$PaymentError_PersistErrorImplCopyWith<$Res> {
  __$$PaymentError_PersistErrorImplCopyWithImpl(
      _$PaymentError_PersistErrorImpl _value, $Res Function(_$PaymentError_PersistErrorImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PaymentError_PersistErrorImpl extends PaymentError_PersistError {
  const _$PaymentError_PersistErrorImpl() : super._();

  @override
  String toString() {
    return 'PaymentError.persistError()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PaymentError_PersistErrorImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class PaymentError_PersistError extends PaymentError {
  const factory PaymentError_PersistError() = _$PaymentError_PersistErrorImpl;
  const PaymentError_PersistError._() : super._();
}

/// @nodoc
abstract class _$$PaymentError_RefundedImplCopyWith<$Res> {
  factory _$$PaymentError_RefundedImplCopyWith(
          _$PaymentError_RefundedImpl value, $Res Function(_$PaymentError_RefundedImpl) then) =
      __$$PaymentError_RefundedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err, String refundTxId});
}

/// @nodoc
class __$$PaymentError_RefundedImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_RefundedImpl>
    implements _$$PaymentError_RefundedImplCopyWith<$Res> {
  __$$PaymentError_RefundedImplCopyWithImpl(
      _$PaymentError_RefundedImpl _value, $Res Function(_$PaymentError_RefundedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
    Object? refundTxId = null,
  }) {
    return _then(_$PaymentError_RefundedImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
      refundTxId: null == refundTxId
          ? _value.refundTxId
          : refundTxId // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentError_RefundedImpl extends PaymentError_Refunded {
  const _$PaymentError_RefundedImpl({required this.err, required this.refundTxId}) : super._();

  @override
  final String err;
  @override
  final String refundTxId;

  @override
  String toString() {
    return 'PaymentError.refunded(err: $err, refundTxId: $refundTxId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentError_RefundedImpl &&
            (identical(other.err, err) || other.err == err) &&
            (identical(other.refundTxId, refundTxId) || other.refundTxId == refundTxId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err, refundTxId);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentError_RefundedImplCopyWith<_$PaymentError_RefundedImpl> get copyWith =>
      __$$PaymentError_RefundedImplCopyWithImpl<_$PaymentError_RefundedImpl>(this, _$identity);
}

abstract class PaymentError_Refunded extends PaymentError {
  const factory PaymentError_Refunded({required final String err, required final String refundTxId}) =
      _$PaymentError_RefundedImpl;
  const PaymentError_Refunded._() : super._();

  String get err;
  String get refundTxId;
  @JsonKey(ignore: true)
  _$$PaymentError_RefundedImplCopyWith<_$PaymentError_RefundedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentError_SendErrorImplCopyWith<$Res> {
  factory _$$PaymentError_SendErrorImplCopyWith(
          _$PaymentError_SendErrorImpl value, $Res Function(_$PaymentError_SendErrorImpl) then) =
      __$$PaymentError_SendErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$PaymentError_SendErrorImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_SendErrorImpl>
    implements _$$PaymentError_SendErrorImplCopyWith<$Res> {
  __$$PaymentError_SendErrorImplCopyWithImpl(
      _$PaymentError_SendErrorImpl _value, $Res Function(_$PaymentError_SendErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$PaymentError_SendErrorImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentError_SendErrorImpl extends PaymentError_SendError {
  const _$PaymentError_SendErrorImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'PaymentError.sendError(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentError_SendErrorImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentError_SendErrorImplCopyWith<_$PaymentError_SendErrorImpl> get copyWith =>
      __$$PaymentError_SendErrorImplCopyWithImpl<_$PaymentError_SendErrorImpl>(this, _$identity);
}

abstract class PaymentError_SendError extends PaymentError {
  const factory PaymentError_SendError({required final String err}) = _$PaymentError_SendErrorImpl;
  const PaymentError_SendError._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$PaymentError_SendErrorImplCopyWith<_$PaymentError_SendErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentError_SignerErrorImplCopyWith<$Res> {
  factory _$$PaymentError_SignerErrorImplCopyWith(
          _$PaymentError_SignerErrorImpl value, $Res Function(_$PaymentError_SignerErrorImpl) then) =
      __$$PaymentError_SignerErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String err});
}

/// @nodoc
class __$$PaymentError_SignerErrorImplCopyWithImpl<$Res>
    extends _$PaymentErrorCopyWithImpl<$Res, _$PaymentError_SignerErrorImpl>
    implements _$$PaymentError_SignerErrorImplCopyWith<$Res> {
  __$$PaymentError_SignerErrorImplCopyWithImpl(
      _$PaymentError_SignerErrorImpl _value, $Res Function(_$PaymentError_SignerErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? err = null,
  }) {
    return _then(_$PaymentError_SignerErrorImpl(
      err: null == err
          ? _value.err
          : err // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PaymentError_SignerErrorImpl extends PaymentError_SignerError {
  const _$PaymentError_SignerErrorImpl({required this.err}) : super._();

  @override
  final String err;

  @override
  String toString() {
    return 'PaymentError.signerError(err: $err)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentError_SignerErrorImpl &&
            (identical(other.err, err) || other.err == err));
  }

  @override
  int get hashCode => Object.hash(runtimeType, err);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentError_SignerErrorImplCopyWith<_$PaymentError_SignerErrorImpl> get copyWith =>
      __$$PaymentError_SignerErrorImplCopyWithImpl<_$PaymentError_SignerErrorImpl>(this, _$identity);
}

abstract class PaymentError_SignerError extends PaymentError {
  const factory PaymentError_SignerError({required final String err}) = _$PaymentError_SignerErrorImpl;
  const PaymentError_SignerError._() : super._();

  String get err;
  @JsonKey(ignore: true)
  _$$PaymentError_SignerErrorImplCopyWith<_$PaymentError_SignerErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
