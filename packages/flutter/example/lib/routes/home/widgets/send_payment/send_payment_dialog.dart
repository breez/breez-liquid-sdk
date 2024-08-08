import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';

class SendPaymentDialog extends StatefulWidget {
  final BindingLiquidSdk liquidSdk;
  final String? barcodeValue;

  const SendPaymentDialog({super.key, required this.liquidSdk, this.barcodeValue});

  @override
  State<SendPaymentDialog> createState() => _SendPaymentDialogState();
}

class _SendPaymentDialogState extends State<SendPaymentDialog> {
  final TextEditingController invoiceController = TextEditingController();

  bool paymentInProgress = false;

  SendPaymentRequest? sendPaymentReq;

  @override
  void initState() {
    super.initState();
    if (widget.barcodeValue != null) {
      invoiceController.text = widget.barcodeValue!;
    }
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: paymentInProgress ? null : const Text("Send Payment"),
      content: paymentInProgress
          ? Padding(
              padding: const EdgeInsets.symmetric(vertical: 16),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text('${sendPaymentReq == null ? "Preparing" : "Sending"} payment...'),
                  const SizedBox(height: 16),
                  const CircularProgressIndicator(color: Colors.blue),
                ],
              ),
            )
          : sendPaymentReq != null
              ? Padding(
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(
                        'Please confirm that you agree to the payment fee of ${sendPaymentReq!.prepareResponse.feesSat} sats.',
                      ),
                    ],
                  ),
                )
              : TextField(
                  decoration: InputDecoration(
                    label: const Text("Enter Invoice"),
                    suffixIcon: IconButton(
                      icon: const Icon(Icons.paste, color: Colors.blue),
                      onPressed: () async {
                        final clipboardData = await Clipboard.getData('text/plain');
                        if (clipboardData != null && clipboardData.text != null) {
                          invoiceController.text = clipboardData.text!;
                        }
                      },
                    ),
                  ),
                  controller: invoiceController,
                ),
      actions: paymentInProgress
          ? []
          : [
              TextButton(
                child: const Text("CANCEL"),
                onPressed: () {
                  Navigator.of(context).pop();
                },
              ),
              sendPaymentReq == null
                  ? TextButton(
                      onPressed: () async {
                        try {
                          setState(() => paymentInProgress = true);
                          PrepareSendRequest prepareSendReq = PrepareSendRequest(
                            destination: invoiceController.text,
                          );
                          PrepareSendResponse req = await widget.liquidSdk.prepareSendPayment(
                            req: prepareSendReq,
                          );
                          debugPrint(
                            "PrepareSendResponse for  ${req.destination}, fees: ${req.feesSat}",
                          );
                          setState(() => sendPaymentReq = SendPaymentRequest(prepareResponse: req));
                        } catch (e) {
                          final errMsg = "Error preparing payment: $e";
                          debugPrint(errMsg);
                          if (context.mounted) {
                            Navigator.pop(context);
                            final snackBar = SnackBar(
                              behavior: SnackBarBehavior.floating,
                              content: Text(errMsg),
                            );
                            ScaffoldMessenger.of(context).showSnackBar(snackBar);
                          }
                        } finally {
                          setState(() => paymentInProgress = false);
                        }
                      },
                      child: const Text("OK"),
                    )
                  : TextButton(
                      onPressed: () async {
                        try {
                          setState(() => paymentInProgress = true);
                          SendPaymentResponse resp = await widget.liquidSdk.sendPayment(req: sendPaymentReq!);
                          debugPrint("Paid ${resp.payment.txId}");
                          if (context.mounted) {
                            Navigator.pop(context);
                          }
                        } catch (e) {
                          final errMsg = "Error sending payment: $e";
                          debugPrint(errMsg);
                          if (context.mounted) {
                            Navigator.pop(context);
                            final snackBar = SnackBar(
                              behavior: SnackBarBehavior.floating,
                              content: Text(errMsg),
                            );
                            ScaffoldMessenger.of(context).showSnackBar(snackBar);
                          }
                        } finally {
                          setState(() => paymentInProgress = false);
                        }
                      },
                      child: const Text("CONFIRM"),
                    ),
            ],
    );
  }
}
